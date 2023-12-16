use std::{collections::HashMap, collections::VecDeque, fs::read_to_string};

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

fn energize_beam_paths(
    starting_position: (usize, usize, usize),
    map: &Vec<Vec<String>>,
    beams: &mut HashMap<(usize, usize), Vec<usize>>,
) {
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back(starting_position);
    let size_x = map.len();
    let size_y = map[0].len();
    while !queue.is_empty() {
        let (cur_x, cur_y, cur_direction) = queue.pop_front().unwrap();
        // if we have encountered this tile with the same direction we ingore it
        // else we store it and its direction and conitnue our BFS approach on traversing the beams paths
        match beams.get_mut(&(cur_x, cur_y)) {
            Some(k) => {
                if k.iter().any(|d| *d == cur_direction) {
                    continue;
                } else {
                    k.push(cur_direction);
                }
            }
            None => {
                beams.insert((cur_x, cur_y), vec![cur_direction]);
            }
        }

        match map[cur_x][cur_y].as_str() {
            "|" => {
                if cur_direction == 1 || cur_direction == 3 {
                    push_next(cur_direction, cur_x, cur_y, &mut queue, size_x, size_y);
                } else {
                    for next_direction in [1, 3] {
                        push_next(next_direction, cur_x, cur_y, &mut queue, size_x, size_y);
                    }
                }
            }
            "-" => {
                if cur_direction == 0 || cur_direction == 2 {
                    push_next(cur_direction, cur_x, cur_y, &mut queue, size_x, size_y);
                } else {
                    for next_direction in [0, 2] {
                        push_next(next_direction, cur_x, cur_y, &mut queue, size_x, size_y);
                    }
                }
            }
            "\\" => {
                let next_direction;
                match cur_direction {
                    0 => next_direction = 1,
                    1 => next_direction = 0,
                    2 => next_direction = 3,
                    3 => next_direction = 2,
                    _ => unreachable!(),
                }
                push_next(next_direction, cur_x, cur_y, &mut queue, size_x, size_y);
            }
            "/" => {
                let next_direction;
                match cur_direction {
                    0 => next_direction = 3,
                    1 => next_direction = 2,
                    2 => next_direction = 1,
                    3 => next_direction = 0,
                    _ => unreachable!(),
                }
                push_next(next_direction, cur_x, cur_y, &mut queue, size_x, size_y);
            }
            "." => {
                push_next(cur_direction, cur_x, cur_y, &mut queue, size_x, size_y);
            }
            _ => unreachable!(),
        }
    }

    fn push_next(
        direction: usize,
        cur_x: usize,
        cur_y: usize,
        queue: &mut VecDeque<(usize, usize, usize)>,
        size_x: usize,
        size_y: usize,
    ) {
        let steps: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (step_x, step_y) = steps[direction];
        let next_x: i32 = cur_x as i32 + step_x;
        let next_y: i32 = cur_y as i32 + step_y;
        if next_x >= 0 && next_x < (size_x as i32) && next_y >= 0 && next_y < (size_y as i32) {
            queue.push_back((next_x as usize, next_y as usize, direction));
        }
    }
}

fn possible_starts(size_x: usize, size_y: usize) -> Vec<(usize, usize, usize)> {
    let mut possible_starts: Vec<(usize, usize, usize)> = Vec::new();
    for x in 0..size_x {
        possible_starts.push((x, 0, 0));
        possible_starts.push((x, size_y - 1, 2));
    }
    for y in 0..size_y {
        possible_starts.push((0, y, 1));
        possible_starts.push((size_x - 1, y, 3));
    }
    possible_starts
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let map = parse_input(input);
    let mut beams: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    energize_beam_paths((0, 0, 0), &map, &mut beams);
    beams.len() as u64
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let map = parse_input(input);
    let all_possible_starts: Vec<(usize, usize, usize)> = possible_starts(map.len(), map[0].len());
    *all_possible_starts
        .iter()
        .map(|starting_position| {
            let mut beams: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
            energize_beam_paths(*starting_position, &map, &mut beams);
            beams.len() as u64
        })
        .collect::<Vec<u64>>()
        .iter()
        .max()
        .unwrap()
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: u64 = solve_part_a(&input);
    let result_part_b: u64 = solve_part_b(&input);
    println!("Part A result: {}", result_part_a);
    println!("Part B result: {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(46, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(51, solve_part_b(&example));
    }
}
