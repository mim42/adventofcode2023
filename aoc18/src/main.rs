use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
fn parse_input_a(input: &Vec<String>) -> Vec<(String, u64)> {
    input
        .iter()
        .map(|line| {
            let s = line.split(" ").collect::<Vec<&str>>();
            (s[0].to_string(), s[1].to_string().parse::<u64>().unwrap())
        })
        .collect::<Vec<(String, u64)>>()
}

fn parse_input_b(input: &Vec<String>) -> Vec<(String, u64)> {
    input
        .iter()
        .map(|line| {
            let s = line.split(" ").collect::<Vec<&str>>();
            let hex = s[2].replace("(#", "").replace(")", "");
            let a = u64::from_str_radix(&hex[..5], 16).unwrap();
            let b = match &hex[5..] {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => unreachable!(),
            };
            (b.to_string(), a)
        })
        .collect::<Vec<(String, u64)>>()
}

fn calculate_area(instructions: Vec<(String, u64)>) -> u64 {
    let mut current_x: i64 = 0;
    let mut current_y: i64 = 0;
    let mut vertices: Vec<(i64, i64)> = Vec::new();
    let mut counter = 0;
    for (direction, step) in instructions {
        counter += step;
        match direction.as_str() {
            "R" => {
                current_y = current_y + step as i64;
                vertices.push((current_x, current_y));
            }
            "L" => {
                current_y = current_y - step as i64;
                vertices.push((current_x, current_y));
            }
            "U" => {
                current_x = current_x - step as i64;
                vertices.push((current_x, current_y));
            }
            "D" => {
                current_x = current_x + step as i64;
                vertices.push((current_x, current_y));
            }
            _ => (),
        }
    }
    let mut result: i64 = 0;
    let length = vertices.len();
    for i in 0..length {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % length];
        result += (x1 * y2) - (y1 * x2);
    }

    (i64::abs(result) as u64 + counter) / 2 + 1
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let instructions = parse_input_a(input);
    calculate_area(instructions)
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let instructions = parse_input_b(input);
    calculate_area(instructions)
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
        assert_eq!(62, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(952408144115, solve_part_b(&example));
    }
}
