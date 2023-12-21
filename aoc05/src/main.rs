use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_seeds(input: &Vec<String>) -> Vec<i64> {
    input
        .get(0)
        .unwrap()
        .clone()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_maps(input: &Vec<String>) -> Vec<Vec<(i64, i64, i64)>> {
    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut current_map: Vec<(i64, i64, i64)> = Vec::new();
    for line in &input[1..] {
        if line.contains(" ") && !line.contains("map") {
            let split = line
                .split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            current_map.push((split[0], split[1], split[2]));
        } else if current_map.len() > 0 {
            maps.push(current_map.clone());
            current_map.clear();
        }
    }
    maps.push(current_map);
    maps
}

fn transform(input: i64, map: &Vec<(i64, i64, i64)>) -> i64 {
    for &(dest, source, size) in map {
        if (source..(source + size)).contains(&input) {
            return dest + (input - source);
        }
    }
    input
}

fn transform_rev(input: i64, map: &Vec<(i64, i64, i64)>) -> i64 {
    for &(dest, source, size) in map {
        if (dest..(dest + size)).contains(&input) {
            return source + (input - dest);
        }
    }
    input
}

fn solve_part_a(input: &Vec<String>) -> i64 {
    let seeds: Vec<i64> = parse_seeds(input);
    let maps: Vec<Vec<(i64, i64, i64)>> = parse_maps(input);
    *seeds
        .iter()
        .map(|seed: &i64| {
            let mut trasnformed_seed = seed.clone();
            maps.iter().for_each(|map| {
                trasnformed_seed = transform(trasnformed_seed, map);
            });
            trasnformed_seed
        })
        .collect::<Vec<i64>>()
        .iter()
        .min()
        .unwrap()
}

fn solve_part_b(input: &Vec<String>) -> i64 {
    let seeds: Vec<i64> = parse_seeds(input);
    let maps: Vec<Vec<(i64, i64, i64)>> = parse_maps(input);
    let seed_ranges = seeds
        .chunks(2)
        .map(|s| (s[0], s[1]))
        .collect::<Vec<(i64, i64)>>();
    for i in 0..i64::MAX {
        let mut trasnformed_seed = i;
        maps.iter().rev().for_each(|map| {
            trasnformed_seed = transform_rev(trasnformed_seed, map);
        });
        if seed_ranges
            .iter()
            .any(|s| ((s.0)..(s.0) + s.1).contains(&trasnformed_seed))
        {
            return i;
        }
    }
    0
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
        assert_eq!(35, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(46, solve_part_b(&example));
    }
}
