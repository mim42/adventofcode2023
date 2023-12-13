use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<Vec<String>>> {
    let mut pattern: Vec<Vec<String>> = Vec::new();
    let mut all_patterns: Vec<Vec<Vec<String>>> = Vec::new();
    for line in input {
        if line == "" {
            all_patterns.push(pattern.clone());
            pattern.clear();
        } else {
            pattern.push(line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        }
    }
    all_patterns.push(pattern);
    all_patterns
}

fn find_reflection(pattern: &Vec<Vec<String>>, errors: u64) -> (u64, u64) {
    for row_index in 0..pattern.len() - 1 {
        let remaining = usize::min(row_index + 1, pattern.len() - row_index - 1);
        let mut counter = 0;
        for offset in 0..remaining {
            for j in 0..pattern[0].len() {
                if pattern[row_index - offset][j] != pattern[row_index + 1 + offset][j] {
                    counter += 1;
                }
            }
        }
        if counter == errors {
            return (1, row_index as u64 + 1);
        }
    }

    for column_index in 0..pattern[0].len() - 1 {
        let remaining = usize::min(column_index + 1, pattern[0].len() - column_index - 1);
        let mut counter = 0;
        for offset in 0..remaining {
            for j in 0..pattern.len() {
                if pattern[j][column_index - offset] != pattern[j][column_index + 1 + offset] {
                    counter += 1;
                }
            }
        }
        if counter == errors {
            return (0, column_index as u64 + 1);
        }
    }
    unreachable!()
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let all_patterns = parse_input(input);
    let mut sum = 0;
    for pattern in &all_patterns {
        let (direction, number) = find_reflection(pattern, 0);
        sum += (direction * 99 * number) + number;
    }
    sum
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let all_patterns = parse_input(input);
    let mut sum = 0;
    for pattern in &all_patterns {
        let (direction, number) = find_reflection(pattern, 1);
        sum += (direction * 99 * number) + number;
    }
    sum
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: u64 = solve_part_a(&input);
    println!("Part A result: {}", result_part_a);
    let result_part_b: u64 = solve_part_b(&input);
    println!("Part B result: {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(405, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(400, solve_part_b(&example));
    }
}
