use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_line(line: &String) -> Vec<i64> {
    line.split(" ")
        .filter(|s| *s != "")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn split_line(input: &Vec<String>) -> Vec<String> {
    input
        .iter()
        .map(|l| {
            (*l.split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .get(1)
                .unwrap())
            .clone()
        })
        .collect::<Vec<String>>()
}

fn parse_input_a(input: &Vec<String>) -> Vec<(i64, i64)> {
    let s = split_line(input);

    parse_line(&s[0])
        .into_iter()
        .zip(parse_line(&s[1]))
        .collect::<Vec<(i64, i64)>>()
}

fn parse_input_b(input: &Vec<String>) -> (i64, i64) {
    let mut s = split_line(input);
    s[0].retain(|c| !c.is_whitespace());
    s[1].retain(|c| !c.is_whitespace());

    (s[0].parse::<i64>().unwrap(), s[1].parse::<i64>().unwrap())
}

fn solve_quadratic(time: &i64, distance: &i64) -> (i64, i64) {
    let time = *time as f64;
    let distance = *distance as f64;
    (
        (0.5 * (time - f64::sqrt(f64::powi(time, 2) - 4.0 * distance))).ceil() as i64,
        (0.5 * (time + f64::sqrt(f64::powi(time, 2) - 4.0 * distance))).floor() as i64,
    )
}

fn solve_part_a(input: &Vec<String>) -> i64 {
    let races = parse_input_a(input);
    races
        .iter()
        .map(|(time, distance)| {
            let (floor, ceil) = solve_quadratic(time, &(distance + 1));
            ceil - floor + 1
        })
        .collect::<Vec<i64>>()
        .iter()
        .product::<i64>()
}

fn solve_part_b(input: &Vec<String>) -> i64 {
    let (time, distance) = parse_input_b(input);
    let (floor, ceil) = solve_quadratic(&time, &(distance + 1));
    ceil - floor + 1
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: i64 = solve_part_a(&input);
    let result_part_b: i64 = solve_part_b(&input);
    println!("Part A result: {}", result_part_a);
    println!("Part B result: {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(288, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(71503, solve_part_b(&example));
    }
}
