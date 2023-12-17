use std::{collections::HashMap, collections::HashSet, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
fn parse_input(input: &Vec<String>) -> Vec<Vec<u64>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|s| s.to_string().parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
}
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    direction: usize,
    straight_length: usize,
}

fn generate_next_points_a(point: &Point, length_x: usize, length_y: usize) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let steps: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for dir in 0..4 {
        let (step_x, step_y) = steps[dir];
        let mut next_length = point.straight_length + 1;
        if dir != point.direction {
            next_length = 1;
        }
        let next_x = point.x as i32 + step_x;
        let next_y = point.y as i32 + step_y;
        if next_x >= (length_x as i32) || next_x < 0 || next_y >= (length_y as i32) || next_y < 0 {
            continue;
        }
        if next_length > 3 {
            continue;
        }
        if i32::abs(dir as i32 - point.direction as i32) == 2 {
            continue;
        }
        let next_point = Point {
            x: next_x as usize,
            y: next_y as usize,
            direction: dir,
            straight_length: next_length,
        };
        points.push(next_point);
    }
    points
}

fn generate_next_points_b(point: &Point, length_x: usize, length_y: usize) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let steps: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for dir in 0..4 {
        if i32::abs(dir as i32 - point.direction as i32) == 2
            || i32::abs(dir as i32 - point.direction as i32) == 0
        {
            continue;
        }
        let (step_x, step_y) = steps[dir];
        for i in 0..7 {
            let next_x = point.x as i32 + step_x * 4 + step_x * i;
            let next_y = point.y as i32 + step_y * 4 + step_y * i;
            if next_x >= (length_x as i32)
                || next_x < 0
                || next_y >= (length_y as i32)
                || next_y < 0
            {
                continue;
            }

            let next_point = Point {
                x: next_x as usize,
                y: next_y as usize,
                direction: dir,
                straight_length: 0,
            };
            points.push(next_point);
        }
    }
    points
}

fn find_shortest(map: &Vec<Vec<u64>>, part: &str) -> u64 {
    let mut unvisited: HashSet<Point> = HashSet::new();
    let mut visited_cost: HashMap<Point, u64> = HashMap::new();
    let mut visited: HashSet<Point> = HashSet::new();

    let starting_point = Point {
        x: 0,
        y: 0,
        direction: 1,
        straight_length: 0,
    };
    unvisited.insert(starting_point);
    visited_cost.insert(starting_point.clone(), 0);

    loop {
        let mut current: Point = Point {
            x: 0,
            y: 0,
            direction: 0,
            straight_length: 0,
        };
        let mut min = 100000;
        for i in &unvisited {
            let a = visited_cost.get(&i).unwrap();
            if *a < min {
                min = *a;
                current = i.clone();
            }
        }

        unvisited.remove(&current);

        let current_cost = *visited_cost.get(&current).unwrap();

        if current.x == map.len() - 1 && current.y == map[0].len() - 1 {
            return current_cost;
        }

        let mut next_points = generate_next_points_a(&current, map.len(), map[0].len());
        if part == "B" {
            next_points = generate_next_points_b(&current, map.len(), map[0].len());
        }

        for next_point in next_points {
            let mut next_point_cost = current_cost;
            for i in (current.x + 1)..(next_point.x + 1) {
                next_point_cost += map[i][current.y];
            }
            for i in (next_point.x)..(current.x) {
                next_point_cost += map[i][current.y];
            }
            for i in (current.y + 1)..(next_point.y + 1) {
                next_point_cost += map[current.x][i];
            }
            for i in (next_point.y)..(current.y) {
                next_point_cost += map[current.x][i];
            }

            if visited.contains(&next_point) {
                continue;
            }
            visited_cost
                .entry(next_point.clone())
                .and_modify(|cost| {
                    if *cost > next_point_cost {
                        *cost = next_point_cost;
                    }
                })
                .or_insert(next_point_cost);

            unvisited.insert(next_point);
        }
        visited.insert(current);
    }
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let map = parse_input(input);
    let a = find_shortest(&map, "A");
    a
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let map = parse_input(input);
    let a = find_shortest(&map, "B");
    a
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
        assert_eq!(102, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(94, solve_part_b(&example));
    }
}
