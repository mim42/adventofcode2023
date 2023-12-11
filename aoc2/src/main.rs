use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    let red: i32 = 12;
    let green: i32 = 13;
    let blue: i32 = 14;
    input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let s = *line.split(":").collect::<Vec<&str>>().get(1).unwrap();
            for draw in s.replace(";", ",").split(",") {
                let seg = draw.split(" ").collect::<Vec<&str>>();
                match *seg.get(2).unwrap() {
                    "red" => {
                        if seg.get(1).unwrap().parse::<i32>().unwrap() > red {
                            return 0;
                        }
                    }
                    "green" => {
                        if seg.get(1).unwrap().parse::<i32>().unwrap() > green {
                            return 0;
                        }
                    }
                    "blue" => {
                        if seg.get(1).unwrap().parse::<i32>().unwrap() > blue {
                            return 0;
                        }
                    }
                    _ => (),
                }
            }
            i as i32 + 1
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn solve_part_b(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| {
            let mut red: i32 = 0;
            let mut green: i32 = 0;
            let mut blue: i32 = 0;
            let s = *line.split(":").collect::<Vec<&str>>().get(1).unwrap();
            for draw in s.replace(";", ",").split(",") {
                let seg = draw.split(" ").collect::<Vec<&str>>();
                match *seg.get(2).unwrap() {
                    "red" => red = red.max(seg.get(1).unwrap().parse::<i32>().unwrap()),
                    "green" => green = green.max(seg.get(1).unwrap().parse::<i32>().unwrap()),
                    "blue" => blue = blue.max(seg.get(1).unwrap().parse::<i32>().unwrap()),
                    _ => (),
                }
            }
            red * green * blue
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
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
        assert_eq!(8, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(2286, solve_part_b(&example));
    }
}
