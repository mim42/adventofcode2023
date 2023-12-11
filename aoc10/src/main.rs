use core::panic;
use std::{collections::HashMap, collections::HashSet, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
fn parse_input(input: &Vec<String>) -> Vec<Vec<String>> {
    input
        .iter()
        .map(|line| line.chars().map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}
fn build_map(space_array: &Vec<Vec<String>>) -> HashMap<(usize, usize), String> {
    let mut map: HashMap<(usize, usize), String> = HashMap::new();
    for (i, a) in space_array.iter().enumerate() {
        for (j, b) in a.iter().enumerate() {
            map.insert((i, j), b.clone());
        }
    }
    map
}
fn starting_point(map: &HashMap<(usize, usize), String>) -> (usize, usize) {
    let (point, _) = map
        .iter()
        .filter(|(_, v)| *v == "S")
        .nth(0)
        .unwrap()
        .clone();
    *point
}

fn find_loop(
    starting_point: (usize, usize),
    map: &HashMap<(usize, usize), String>,
) -> HashSet<(usize, usize)> {
    let steps = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut loop_points: HashSet<(usize, usize)> = HashSet::new();
    let (mut cur_x, mut cur_y) = (starting_point.0 + 1, starting_point.1);
    let mut direction: usize = 1;
    loop_points.insert((cur_x, cur_y));
    while (cur_x, cur_y) != starting_point {
        match map.get(&(cur_x, cur_y)).unwrap().as_str() {
            "J" => {
                if direction == 0 {
                    direction = 3;
                } else if direction == 1 {
                    direction = 2;
                }
            }
            "|" => {}
            "F" => {
                if direction == 3 {
                    direction = 0;
                } else if direction == 2 {
                    direction = 1;
                }
            }
            "-" => {}
            "L" => {
                if direction == 1 {
                    direction = 0;
                } else if direction == 2 {
                    direction = 3;
                }
            }
            "7" => {
                if direction == 0 {
                    direction = 1;
                } else if direction == 3 {
                    direction = 2;
                }
            }
            _ => panic!(),
        }
        let (next_x, next_y) = steps[direction];
        (cur_x, cur_y) = (
            (cur_x as i64 + next_x) as usize,
            (cur_y as i64 + next_y) as usize,
        );
        loop_points.insert((cur_x, cur_y));
    }
    loop_points
}
fn solve_part_a(input: &Vec<String>) -> u64 {
    let map = build_map(&parse_input(input));
    let (x, y) = starting_point(&map);
    let length = find_loop((x, y), &map).len();
    length as u64 / 2
}

fn solve_part_b(input: &Vec<String>) -> i64 {
    let map = build_map(&parse_input(input));
    let (x, y) = starting_point(&map);
    let loop_points = find_loop((x, y), &map);
    let mut space_counter = 0;

    // go through all the points not in the loop
    for ((x, y), _) in map.iter().filter(|(k, _)| !loop_points.contains(k)) {
        // keep a counter for ray casting algorithm
        let mut counter = 0;
        let mut prev_character = "".to_string();

        (0..*x).for_each(|i| {
            let mut character = map.get(&(i, *y)).unwrap().clone();
            // special condition for S could different on other inputs
            if character == "S".to_string() {
                character = "7".to_string();
            }
            if loop_points.contains(&(i, *y)) {
                // dont double count edge cases
                if prev_character == "F" && character == "J"
                    || prev_character == "7" && character == "L"
                {
                    counter -= 1;
                }
                // for everything else increase counter
                if character != "|".to_string() {
                    counter += 1;
                }
                // save the last special character to know edge cases
                if "FJ7L".contains(&character) {
                    prev_character = character.clone()
                }
            }
        });
        if counter % 2 != 0 {
            space_counter += 1
        }
    }

    space_counter
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: u64 = solve_part_a(&input);
    let result_part_b: i64 = solve_part_b(&input);
    println!("Part A result: {}", result_part_a);
    println!("Part B result: {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(8, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example1() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(10, solve_part_b(&example));
    }
}
