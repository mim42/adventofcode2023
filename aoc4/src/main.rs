use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_input(input: &String) -> (Vec<i32>, Vec<i32>) {
    let a = input
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .to_string();
    let b = a.split(" |").collect::<Vec<&str>>();
    let winnings: Vec<i32> = b
        .get(0)
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|c| {
            c.iter()
                .collect::<String>()
                .to_string()
                .trim()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();
    let numbers: Vec<i32> = b
        .get(1)
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|c| {
            c.iter()
                .collect::<String>()
                .to_string()
                .trim()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();
    (winnings, numbers)
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|i| {
            let (winning, numbers) = parse_input(i);
            let winning = winning.into_iter().collect::<HashSet<i32>>();
            let matches: i32 = numbers
                .iter()
                .map(|n| if winning.contains(n) { 1 } else { 0 })
                .collect::<Vec<i32>>()
                .iter()
                .sum();
            if matches > 0 {
                i32::pow(2, matches as u32 - 1)
            } else {
                0
            }
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn solve_part_b(input: &Vec<String>) -> i32 {
    let mut scratchcards: HashMap<i32, i32> = (0..input.len())
        .map(|k| (k as i32, 1))
        .collect::<HashMap<i32, i32>>();
    for (i, card) in input.iter().enumerate() {
        let (winning, numbers) = parse_input(card);
        let winning = winning.into_iter().collect::<HashSet<i32>>();
        let matches: i32 = numbers
            .iter()
            .map(|n| if winning.contains(n) { 1 } else { 0 })
            .collect::<Vec<i32>>()
            .iter()
            .sum();
        if matches > 0 {
            let num = *scratchcards.get(&(i as i32)).unwrap();
            for j in 1..matches + 1 {
                scratchcards.entry(i as i32 + j).and_modify(|v| *v += num);
            }
        }
    }
    scratchcards.iter().map(|(_, v)| v).sum()
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
        assert_eq!(13, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(30, solve_part_b(&example));
    }
}
