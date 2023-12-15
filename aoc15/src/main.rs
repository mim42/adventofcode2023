use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
fn parse_input(input: &Vec<String>) -> Vec<String> {
    input[0]
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn hash(input: &String) -> u32 {
    let mut result = 0;
    for i in input.chars() {
        let num = i as u32;
        result = ((result + num) * 17) % 256;
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> u32 {
    let steps = parse_input(input);
    steps
        .iter()
        .map(|k| hash(k))
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn solve_part_b(input: &Vec<String>) -> u32 {
    let steps = parse_input(input);
    let mut boxes: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];
    for step in steps {
        match step.find("=") {
            Some(j) => {
                let instr = step[..j].to_string();
                let focal_length = step[j + 1..].parse::<u32>().unwrap();
                let index = hash(&instr) as usize;
                let slot = &mut boxes[index];
                match slot.iter().position(|(x, _)| *x == instr) {
                    Some(k) => slot[k] = (instr, focal_length),
                    None => slot.push((instr, focal_length)),
                }
            }
            None => {
                let instr = step[..step.len() - 1].to_string();
                let index = hash(&instr) as usize;
                let slot = &mut boxes[index];
                slot.retain(|(x, _)| *x != instr);
            }
        }
    }
    let mut result = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, (_, focal)) in b.iter().enumerate() {
            result += (i + 1) as u32 * (j + 1) as u32 * focal
        }
    }
    result
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: u32 = solve_part_a(&input);
    println!("Part A result: {}", result_part_a);
    let result_part_b: u32 = solve_part_b(&input);
    println!("Part B result: {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(1320, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(145, solve_part_b(&example));
    }
}
