use std::{collections::HashMap, fs::read_to_string};

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
#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn generate_next_points(point: &Point) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let steps: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for dir in 0..4 {
        let (step_x, step_y) = steps[dir];

        let next_x = point.x as i64 + step_x;
        let next_y = point.y as i64 + step_y;

        let next_point = Point {
            x: next_x,
            y: next_y,
        };
        points.push(next_point);
    }
    points
}

fn bfs(map: &Vec<Vec<String>>, total_steps: u64) -> u64 {
    let mut hashmap: HashMap<Point, String> = HashMap::new();
    let mut starting_point: Point = Point { x: 0, y: 0 };
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            hashmap.insert(
                Point {
                    x: i as i64,
                    y: j as i64,
                },
                map[i][j].clone(),
            );
            if map[i][j] == "S".to_string() {
                starting_point = Point {
                    x: i as i64,
                    y: j as i64,
                }
            }
        }
    }
    let mut unvisited: Vec<(Point, u64)> = Vec::new();
    let mut visited: HashMap<Point, u64> = HashMap::new();
    unvisited.push((starting_point, 0));

    while !unvisited.is_empty() {
        let (current, current_step) = unvisited.remove(0);
        if current_step > total_steps {
            continue;
        }
        let next_points = generate_next_points(&current);
        for next_point in next_points {
            if !visited.contains_key(&next_point) {
                if !unvisited.iter().any(|(p, _)| *p == next_point) {
                    match hashmap
                        .get(&Point {
                            x: next_point.x.rem_euclid(map.len() as i64),
                            y: next_point.y.rem_euclid(map[0].len() as i64),
                        })
                        .unwrap()
                        .as_str()
                    {
                        "#" => (),
                        _ => unvisited.push((next_point, current_step + 1)),
                    }
                }
            }
        }
        visited.insert(current, current_step);
    }
    // depending on the size of the total_steps, return either the odd or even
    visited
        .iter()
        .filter(|(_, k)| *k % 2 == (total_steps % 2))
        .count() as u64
}

fn solve_part_a(input: &Vec<String>, total_steps: u64) -> u64 {
    let map = parse_input(input);
    bfs(&map, total_steps)
}

fn solve_part_b(input: &Vec<String>, total_steps: u64) -> u64 {
    let map = parse_input(input);
    let length = map.len() as u64;
    let f_0 = bfs(&map, 65) as f64;
    let f_1 = bfs(&map, 65 + length) as f64;
    let f_2 = bfs(&map, 65 + length * 2) as f64;

    // f(x) = ax^2 + bx + c
    // f(0) = c
    // f(1) = a+b + f(0) => a+b = f(1) - (f_0) => 2a + 2b = 2 ( f(1) - f(0) )  (1)
    // f(2) = 4a + 2b + f(0)   (2)
    // subtracting (2) with (1)
    // f(2) - 2 (f(1) - f(0)) - f(0) = 4a - 2a + 2b - 2b
    // 2a = f(2) - 2 f(1) + f(0)
    // a = ( f(2) - 2 f(1) + f(0) ) / 2
    // then from (1)
    // b = f(1) - f(2) - a

    let c = f_0;
    let a = (f_2 - (2.0 * f_1) + f_0) / 2.0;
    let b = f_1 - f_0 - a;

    let x = (total_steps / length) as f64;

    (a * x * x + b * x + c) as u64
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: u64 = solve_part_a(&input, 64);
    let result_part_b: u64 = solve_part_b(&input, 26501365);
    println!("Part A result: {}", result_part_a);
    println!("Part B result: {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(16, solve_part_a(&example, 6));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(16733044, solve_part_b(&example, 5000));
    }
}
