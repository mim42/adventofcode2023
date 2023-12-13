use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
fn parse_input_a(input: &Vec<String>) -> Vec<(String, Vec<usize>)> {
    input
        .iter()
        .map(|line| {
            let mut split = line.split(" ");
            let springs = split.next().unwrap().to_string() + ".";
            let constraints = split
                .next()
                .unwrap()
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (springs, constraints)
        })
        .collect::<Vec<(String, Vec<usize>)>>()
}
fn parse_input_b(input: &Vec<String>) -> Vec<(String, Vec<usize>)> {
    input
        .iter()
        .map(|line| {
            let mut split = line.split(" ");
            let springs = split.next().unwrap().to_string();
            let mut unfolded = springs.clone();
            for _ in 0..4 {
                unfolded = unfolded + "?" + &springs;
            }
            let c = split
                .next()
                .unwrap()
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let unfolded_constaints = [c.as_slice(), &c, &c, &c, &c].concat();
            (unfolded + ".", unfolded_constaints)
        })
        .collect::<Vec<(String, Vec<usize>)>>()
}

fn count_arrangements(
    spring: &str,
    constraints: &[usize],
    cache: &mut HashMap<String, u64>,
) -> u64 {
    let key = constraints
        .iter()
        .map(|c| c.to_string())
        .collect::<String>()
        + &spring;
    match cache.get(&key) {
        Some(result) => return *result,
        None => (),
    }

    if constraints.len() == 0 {
        match spring.find("#") {
            Some(_) => return 0,
            None => return 1,
        }
    }
    if spring.len() == 0 {
        return 0;
    }

    match &spring[..1] {
        "." => return count_arrangements(&spring[1..], constraints, cache),
        "#" => {
            let current_constraint = constraints[0];
            if current_constraint > spring.len() {
                return 0;
            }
            if !spring[..current_constraint]
                .chars()
                .all(|c| c == '#' || c == '?')
            {
                return 0;
            }
            if spring.len() == current_constraint {
                if constraints.len() == 1 {
                    return 1;
                } else {
                    return 0;
                }
            }
            if spring[current_constraint..current_constraint + 1]
                .matches(|c| c == '?' || c == '.')
                .count()
                != 0
            {
                let result =
                    count_arrangements(&spring[current_constraint + 1..], &constraints[1..], cache);
                let key = constraints[1..]
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<String>()
                    + &spring[current_constraint + 1..];
                cache.insert(key, result);
                return result;
            } else {
                return 0;
            }
        }
        "?" => {
            let result = count_arrangements(&spring[1..], constraints, cache);
            let key = constraints
                .iter()
                .map(|c| c.to_string())
                .collect::<String>()
                + &spring[1..];
            cache.insert(key, result);

            let result2 = count_arrangements(
                &("#".to_string() + &spring[1..].to_string()),
                constraints,
                cache,
            );
            let key = constraints
                .iter()
                .map(|c| c.to_string())
                .collect::<String>()
                + "#"
                + &spring[1..];
            cache.insert(key, result2);

            return result + result2;
        }
        _ => (),
    };

    0
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    parse_input_a(input)
        .iter()
        .map(|(spring, constraints)| {
            let mut cache: HashMap<String, u64> = HashMap::new();
            count_arrangements(&spring, &constraints, &mut cache)
        })
        .collect::<Vec<u64>>()
        .iter()
        .sum()
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    parse_input_b(input)
        .iter()
        .map(|(spring, c)| {
            let mut cache: HashMap<String, u64> = HashMap::new();
            count_arrangements(&spring, &c, &mut cache)
        })
        .collect::<Vec<u64>>()
        .iter()
        .sum()
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
        assert_eq!(21, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(525152, solve_part_b(&example));
    }
}
