use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

enum Part {
    A,
    B,
}

fn level_a(cards: &String) -> u64 {
    let mut counter: HashMap<String, u64> = HashMap::new();
    for c in cards.chars() {
        *counter.entry(c.to_string()).or_default() += 1;
    }
    let (_, max) = counter.iter().max_by_key(|(_, v)| *v).unwrap();
    let max = max.clone();
    match max {
        1 => 1,
        2 => 1 + counter.iter().filter(|(_, v)| **v == 2).count() as u64,
        3 => 4 + counter.iter().any(|(_, v)| *v == 2) as u64,
        4 => 6,
        5 => 7,
        _ => 0,
    }
}
fn level_b(cards: &String) -> u64 {
    let mut counter: HashMap<String, u64> = HashMap::new();
    for c in cards.chars() {
        *counter.entry(c.to_string()).or_default() += 1;
    }
    match counter
        .iter()
        .filter(|(k, _)| *k != "J")
        .max_by_key(|(_, v)| *v)
    {
        Some((k, _)) => level_a(&cards.clone().replace("J", k)),
        None => 7,
    }
}

fn value(letter: String, part: &Part) -> u64 {
    let values = match part {
        Part::A => vec![
            "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
        ],
        Part::B => vec![
            "J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A",
        ],
    };
    values.iter().position(|&r| r == letter).unwrap() as u64
}

fn cmp(line: &(String, u64), other: &(String, u64), part: &Part) -> Ordering {
    let (cards, _) = line;
    let (other_cards, _) = other;
    let card_level = match part {
        Part::A => level_a(cards),
        Part::B => level_b(cards),
    };
    let other_cards_level = match part {
        Part::A => level_a(other_cards),
        Part::B => level_b(other_cards),
    };
    if card_level > other_cards_level {
        Ordering::Greater
    } else if card_level < other_cards_level {
        Ordering::Less
    } else {
        let mut other_cards_iter = other_cards.chars();
        for c in cards.chars() {
            let next_other_char = other_cards_iter.next().unwrap().to_string();
            let c_pos = value(c.to_string(), part);
            let o_pos = value(next_other_char.to_string(), part);
            if c_pos == o_pos {
                continue;
            } else if c_pos > o_pos {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

fn parse_line(input: &Vec<String>) -> Vec<(String, u64)> {
    input
        .iter()
        .map(|line| {
            let a = line.split(" ").collect::<Vec<&str>>();
            (a[0].to_string(), a[1].parse::<u64>().unwrap())
        })
        .collect::<Vec<(String, u64)>>()
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let mut hands = parse_line(input);
    hands.sort_by(|a, b| cmp(a, b, &Part::A));
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.1)
        .collect::<Vec<u64>>()
        .iter()
        .sum()
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let mut hands = parse_line(input);
    hands.sort_by(|a, b| cmp(a, b, &Part::B));
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.1)
        .collect::<Vec<u64>>()
        .iter()
        .sum()
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
        assert_eq!(6440, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(5905, solve_part_b(&example));
    }
}
