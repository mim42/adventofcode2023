use std::collections::HashMap;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| {
            let mut first_character: String = String::new();
            let mut last_character: String = String::new();
            for c in line.chars() {
                if c.is_numeric() {
                    first_character = c.to_string();
                    break;
                }
            }
            for c in line.chars().rev() {
                if c.is_numeric() {
                    last_character = c.to_string();
                    break;
                }
            }
            (first_character + &last_character).parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn solve_part_b(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| {
            let reversed_line = line.clone().chars().rev().collect::<String>();
            let patterns = [
                "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7",
                "seven", "8", "eight", "9", "nine",
            ];
            let map = patterns
                .into_iter()
                .enumerate()
                .map(|(i, pattern)| {
                    let index: i32 = match line.find(pattern) {
                        Some(n) => n as i32,
                        None => i32::MAX,
                    };
                    (index, (i as i32 / 2) + 1)
                })
                .collect::<HashMap<i32, i32>>();
            let first_character: String = (map.iter().min_by_key(|s| *s).unwrap().1).to_string();

            let map = patterns
                .into_iter()
                .enumerate()
                .map(|(i, pattern)| {
                    let index: i32 =
                        match reversed_line.find(&pattern.chars().rev().collect::<String>()) {
                            Some(n) => n as i32,
                            None => i32::MAX,
                        };
                    (index, (i as i32 / 2) + 1)
                })
                .collect::<HashMap<i32, i32>>();
            let last_character: String = (map.iter().min_by_key(|s| *s).unwrap().1).to_string();

            (first_character + &last_character).parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: i32 = solve_part_a(&input);
    let result_part_b: i32 = solve_part_b(&input);
    println!("result of part a {}", result_part_a);
    println!("result of part b {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(142, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(281, solve_part_b(&example));
    }
}
