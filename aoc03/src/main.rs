use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn add_padding(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut local_input: Vec<Vec<String>> = input
        .iter()
        .map(|i| i.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    let width = input.get(0).unwrap().len();
    let width_padding: Vec<String> = (0..width).map(|_| ".".to_string()).collect::<Vec<String>>();
    local_input.push(width_padding.clone());
    local_input.insert(0, width_padding);
    for i in local_input.iter_mut() {
        i.insert(0, ".".to_string());
        i.push(".".to_string());
    }
    local_input
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    let mut answer: i32 = 0;
    let input = add_padding(input);
    let mut numbers: Vec<String> = Vec::new();
    let mut valid: bool = false;
    let pos: Vec<(i32, i32)> = vec![
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    for i in 0..input.len() {
        for j in 0..input.get(0).unwrap().len() {
            let current = &input[i][j];
            if current.chars().next().unwrap().is_ascii_digit() {
                numbers.push(current.clone());
                for x in &pos {
                    let f = &input[(i as i32 + x.0) as usize][(j as i32 + x.1) as usize];
                    if !f.chars().next().unwrap().is_ascii_digit() && f != "." {
                        valid = true;
                    }
                }
            } else {
                if valid {
                    answer += numbers.join("").parse::<i32>().unwrap();
                }
                valid = false;
                numbers.clear();
            }
        }
    }
    answer
}

fn solve_part_b(input: &Vec<String>) -> i32 {
    let input = add_padding(input);
    let mut numbers: Vec<String> = Vec::new();
    let mut valid = false;
    let mut gear = (0, 0);
    let mut valid_gear: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let pos: Vec<(i32, i32)> = vec![
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    for i in 0..input.len() {
        for j in 0..input.get(0).unwrap().len() {
            let current = &input[i][j];
            if current.chars().next().unwrap().is_ascii_digit() {
                numbers.push(current.clone());
                for x in &pos {
                    let f = &input[(i as i32 + x.0) as usize][(j as i32 + x.1) as usize];
                    if f == "*" {
                        valid = true;
                        gear = (i as i32 + x.0, j as i32 + x.1);
                    }
                }
            } else {
                if valid {
                    let num = numbers.join("").parse::<i32>().unwrap();
                    valid_gear
                        .entry(gear)
                        .and_modify(|v| v.push(num))
                        .or_insert(vec![num]);
                }
                valid = false;
                numbers.clear();
            }
        }
    }
    valid_gear
        .iter()
        .filter(|&(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().copied().reduce(|a, b| a * b).unwrap())
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: i32 = solve_part_a(&input);
    let result_part_b: i32 = solve_part_b(&input);
    println!("Part A result: {}", result_part_a);
    println!("Part B result: {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(4361, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(467835, solve_part_b(&example));
    }
}
