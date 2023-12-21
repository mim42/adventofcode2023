use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn build_map(input: &[String]) -> HashMap<String, (String, String)> {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    input.iter().for_each(|line| {
        map.insert(
            line[0..3].to_string(),
            (line[7..10].to_string(), line[12..15].to_string()),
        );
    });
    map
}
fn solve_part_a(input: &Vec<String>) -> u64 {
    let instructions = input.get(0).unwrap().clone().chars().collect::<Vec<char>>();
    let map: HashMap<String, (String, String)> = build_map(&input[2..]);
    let mut current_node = "AAA";
    let mut counter: u64 = 0;

    while current_node != "ZZZ" {
        let (left, right) = map.get(current_node).unwrap();
        let instruction = instructions[counter as usize % instructions.len()];
        if instruction == 'L' {
            current_node = left;
        } else {
            current_node = right;
        }
        counter += 1;
    }

    counter
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

// lcm(a,b,c) -> lcm(a,lcm(b,c))
fn lcm(numbers: &[u64]) -> u64 {
    let a = numbers[0];
    let mut b = numbers[1];
    if numbers.len() > 2 {
        b = lcm(&numbers[1..]);
    }
    a * b / gcd(a, b)
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let instructions = input.get(0).unwrap().clone().chars().collect::<Vec<char>>();
    let map: HashMap<String, (String, String)> = build_map(&input[2..]);
    let mut current_nodes: Vec<String> = Vec::new();

    for (node, (_, _)) in &map {
        if node.ends_with("A") {
            current_nodes.push(node.clone());
        }
    }
    let mut results: Vec<u64> = Vec::new();
    for i in &current_nodes {
        let mut counter: u64 = 0;
        let mut node = i;
        while !node.ends_with("Z") {
            let (left, right) = map.get(node).unwrap();
            let instruction = instructions[counter as usize % instructions.len()];
            if instruction == 'L' {
                node = left;
            } else {
                node = right;
            }
            counter += 1;
        }
        results.push(counter);
    }

    lcm(results.as_slice())
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
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(2, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(6, solve_part_b(&example));
    }
}
