use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}
#[derive(Debug, Clone)]
struct Velocity {
    v_x: f64,
    v_y: f64,
}

fn parse_input(input: &Vec<String>) -> Vec<(Point, Velocity)> {
    input
        .iter()
        .map(|line| {
            let mut a = line.split("@");
            let mut point = a.next().unwrap().split(",");
            let x = point.next().unwrap().trim().parse::<f64>().unwrap();
            let y = point.next().unwrap().trim().parse::<f64>().unwrap();
            let mut velocities = a.next().unwrap().split(",");
            let v_x = velocities.next().unwrap().trim().parse::<f64>().unwrap();
            let v_y = velocities.next().unwrap().trim().parse::<f64>().unwrap();
            (Point { x, y }, Velocity { v_x, v_y })
        })
        .collect::<Vec<(Point, Velocity)>>()
}

fn build_linear_function(point: &(Point, Velocity)) -> (f64, f64) {
    let y1 = point.0.y;
    let x1 = point.0.x;
    let y2 = y1 + point.1.v_y;
    let x2 = x1 + point.1.v_x;
    let a = (y2 - y1) / (x2 - x1);
    let b = -1.0 * a * x1 + y1;

    (a, b)
}

fn collides(one: &(Point, Velocity), two: &(Point, Velocity), low: f64, high: f64) -> bool {
    let (a, b) = build_linear_function(one);
    let (d, c) = build_linear_function(two);

    let x = (b - c) / (d - a);
    let y = (b - c) / (d - a) * a + b;

    if x > low && x < high && y > low && y < high {
        // check for times before t=0;
        if one.1.v_x > 0.0 && one.0.x > x || one.1.v_y > 0.0 && one.0.y > y {
            false
        } else if two.1.v_x > 0.0 && two.0.x >= x || two.1.v_y > 0.0 && two.0.y >= y {
            false
        } else if one.1.v_x < 0.0 && one.0.x <= x || one.1.v_y < 0.0 && one.0.y <= y {
            false
        } else if two.1.v_x < 0.0 && two.0.x <= x || two.1.v_y < 0.0 && two.0.y <= y {
            false
        } else {
            true
        }
    } else {
        false
    }
}

fn solve_part_a(input: &Vec<String>, low: f64, high: f64) -> u64 {
    let rocks = parse_input(input);
    let mut counter = 0;
    for i in 0..rocks.len() {
        for j in i + 1..rocks.len() {
            if collides(&rocks[i], &rocks[j], low, high) {
                counter += 1;
            }
        }
    }
    counter
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: u64 = solve_part_a(&input, 200000000000000.0, 400000000000000.0);
    println!("Part A result: {}", result_part_a);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(2, solve_part_a(&example, 7.0, 27.0));
    }
}
