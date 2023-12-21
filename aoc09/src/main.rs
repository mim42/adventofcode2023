use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
fn parse_input(input: &Vec<String>) -> Vec<Vec<i64>> {
    input
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn find_next(seq: &Vec<i64>) -> i64 {
    let mut last_diagonal: Vec<i64> = vec![seq.last().unwrap().clone()];
    let mut temp: Vec<i64> = seq.clone();
    while !temp.iter().all(|a| *a == 0) {
        temp = temp
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<i64>>();
        last_diagonal.push(temp.last().unwrap().clone());
    }
    last_diagonal.iter().rev().fold(0, |acc, x| x + acc)
}

fn solve_part_a(input: &Vec<String>) -> i64 {
    parse_input(input)
        .iter()
        .map(|seq| find_next(seq))
        .collect::<Vec<i64>>()
        .iter()
        .sum()
}

fn solve_part_b(input: &Vec<String>) -> i64 {
    parse_input(input)
        .into_iter()
        .map(|mut seq| {
            seq.reverse();
            find_next(&seq)
        })
        .collect::<Vec<i64>>()
        .iter()
        .sum()
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
        assert_eq!(114, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(2, solve_part_b(&example));
    }
}
