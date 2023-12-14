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
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

fn roll_platform(mut platform: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for i in 1..platform.len() {
        for j in 0..platform[0].len() {
            if platform[i][j] == "O" {
                let mut counter = 0;
                while i - counter > 0 {
                    let next = &platform[i - counter - 1][j];
                    if *next == "O" || *next == "#" {
                        break;
                    }
                    counter += 1;
                }
                if counter > 0 {
                    platform[i][j] = ".".to_string();
                    platform[i - counter][j] = "O".to_string();
                }
            }
        }
    }
    platform
}

fn rotate_platform(mut platform: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let l = platform.len();
    for i in 0..l / 2 {
        for j in i..l - i - 1 {
            let temp = platform[i][j].clone();
            platform[i][j] = platform[l - j - 1][i].clone();
            platform[l - j - 1][i] = platform[l - i - 1][l - j - 1].clone();
            platform[l - i - 1][l - j - 1] = platform[j][l - i - 1].clone();
            platform[j][l - i - 1] = temp;
        }
    }
    platform
}

fn cycle_platform(mut platform: Vec<Vec<String>>, num: u64) -> Vec<Vec<String>> {
    let mut cache: HashMap<String, u64> = HashMap::new();
    let mut i = 0;
    while i < num {
        for _ in 0..4 {
            platform = rotate_platform(roll_platform(platform))
        }
        let key = platform
            .iter()
            .map(|line| line.join(""))
            .collect::<String>();

        match cache.get(&key) {
            Some(k) => {
                i = num - (num - i) % (i - *k);
            }
            None => {
                cache.insert(key, i);
            }
        }
        i += 1;
    }

    platform
}
fn calculate_load(platform: &Vec<Vec<String>>) -> u64 {
    let mut load = 0;
    for i in 0..platform.len() {
        for j in 0..platform[0].len() {
            if platform[i][j] == "O" {
                load += platform.len() - i;
            }
        }
    }
    load as u64
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let map = parse_input(input);
    let roll_map = roll_platform(map);
    calculate_load(&roll_map)
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let mut platform = parse_input(input);
    platform = cycle_platform(platform, 1_000_000_000);
    calculate_load(&platform)
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
        assert_eq!(136, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(64, solve_part_b(&example));
    }
}
