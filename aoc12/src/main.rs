use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
fn parse_input_a(input: &Vec<String>) -> Vec<(String, Vec<u8>)> {
    input
        .iter()
        .map(|line| {
            let mut split = line.split(" ");
            let springs = split.next().unwrap().to_string();
            let constraints = split
                .next()
                .unwrap()
                .split(",")
                .map(|c| c.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            (springs, constraints)
        })
        .collect::<Vec<(String, Vec<u8>)>>()
}
fn parse_input_b(input: &Vec<String>) -> Vec<(String, Vec<u8>)> {
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
                .map(|c| c.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            let unfolded_constaints = [c.as_slice(), &c, &c, &c, &c].concat();
            (unfolded, unfolded_constaints)
        })
        .collect::<Vec<(String, Vec<u8>)>>()
}

fn is_valid(spring: &String, constraints: &Vec<u8>) -> bool {
    let mut index_constraint = 0;
    let mut counter_continuity = 0;
    let spring = spring.clone() + ".";
    for c in spring.chars() {
        if c == '#' {
            if index_constraint == constraints.len() {
                return false;
            }
            counter_continuity += 1;
        } else {
            if counter_continuity != 0 {
                if counter_continuity == *constraints.get(index_constraint).unwrap() {
                    counter_continuity = 0;
                    index_constraint += 1;
                } else {
                    return false;
                }
            }
        }
    }
    if index_constraint == constraints.len() {
        return true;
    }
    if counter_continuity != 0 {
        return counter_continuity == *constraints.get(index_constraint).unwrap();
    }
    false
}

fn is_valid_partial(spring: &String, constraints: &Vec<u8>, index: usize) -> bool {
    let mut index_constraint = 0;
    let mut counter_continuity = 0;
    let spring = spring.clone();
    for c in spring[..index].chars() {
        if c == '#' {
            if index_constraint == constraints.len() {
                return false;
            }
            counter_continuity += 1;
        } else {
            if counter_continuity != 0 {
                if counter_continuity == *constraints.get(index_constraint).unwrap() {
                    counter_continuity = 0;
                    index_constraint += 1;
                } else {
                    return false;
                }
            }
        }
        if counter_continuity != 0 {
            if counter_continuity > *constraints.get(index_constraint).unwrap() {
                return false;
            }
        }
    }

    true
}

fn count_arrangements(spring: &String, constraints: &Vec<u8>) -> u64 {
    let mut arrangements: Vec<String> = vec![spring.clone()];
    let mut index = 0;
    loop {
        let arrangement = arrangements.get(index).unwrap();
        match arrangement.find("?") {
            Some(k) => {
                let s1 = arrangement.replacen("?", "#", 1);
                let s2 = arrangement.replacen("?", ".", 1);
                if is_valid_partial(&s1, constraints, k) {
                    arrangements.push(s1);
                }
                if is_valid_partial(&s2, constraints, k) {
                    arrangements.push(s2);
                }
            }
            None => break,
        }
        index += 1;
    }

    let a = arrangements[index..]
        .iter()
        .filter(|s| is_valid(s, constraints))
        .count() as u64;
    drop(arrangements);
    a
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    parse_input_a(input)
        .iter()
        .map(|(spring, constraints)| count_arrangements(spring, constraints))
        .collect::<Vec<u64>>()
        .iter()
        .sum()
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    parse_input_b(input)
        .iter()
        .map(|(spring, c)| count_arrangements(&spring, &c))
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
