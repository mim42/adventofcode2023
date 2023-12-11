use std::{collections::HashMap, collections::HashSet, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
fn parse_input(input: &Vec<String>) -> Vec<Vec<String>> {
    input
        .iter()
        .map(|line| line.chars().map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}
fn find_expansion_lines(space_array: &Vec<Vec<String>>) -> (Vec<usize>, Vec<usize>) {
    let mut x: Vec<usize> = Vec::new();
    let mut y: Vec<usize> = Vec::new();

    for i in 0..space_array.len() {
        if space_array[i].iter().all(|c| c != "#") {
            x.push(i);
        }
    }
    for i in 0..space_array[0].len() {
        if space_array.iter().all(|x| x[i] != "#") {
            y.push(i);
        }
    }
    (x, y)
}

fn find_galaxies(map: &Vec<Vec<String>>) -> HashSet<(usize, usize)> {
    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, galaxy)| {
            if galaxy == "#" {
                galaxies.insert((i, j));
            }
        })
    });
    galaxies
}

fn find_lengths(
    galaxies: &HashSet<(usize, usize)>,
    expansion_lines: (Vec<usize>, Vec<usize>),
    expansion_coefficient: u64,
) -> Vec<u64> {
    let mut temp: HashMap<(usize, usize, usize, usize), u64> = HashMap::new();
    for (i_x, i_y) in galaxies {
        for (j_x, j_y) in galaxies {
            let ix = *i_x;
            let iy = *i_y;
            let jx = *j_x;
            let jy = *j_y;

            if ix != jx || iy != jy {
                if temp.contains_key(&(ix, iy, jx, jy)) || temp.contains_key(&(jx, jy, ix, iy)) {
                    continue;
                } else {
                    let mut counter_x = 0;
                    let mut counter_y = 0;
                    let (x, y) = &expansion_lines;
                    //check how many times we cross a expansion line in x and y direction
                    for k in x {
                        if (ix..jx).contains(&k) || (jx..ix).contains(&k) {
                            counter_x += 1;
                        }
                    }
                    for k in y {
                        if (iy..jy).contains(&k) || (jy..iy).contains(&k) {
                            counter_y += 1;
                        }
                    }
                    // manhatan distance + the expended lines crossed * the coefficient
                    temp.insert(
                        (ix, iy, jx, jy),
                        (ix as i64).abs_diff(jx as i64)
                            + (iy as i64).abs_diff(jy as i64)
                            + ((counter_x + counter_y) * (expansion_coefficient - 1)),
                    );
                }
            }
        }
    }
    temp.iter().map(|(_, v)| *v).collect::<Vec<u64>>()
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let map = parse_input(input);
    let expansion_lines = find_expansion_lines(&map);
    let galaxies = find_galaxies(&map);
    let lengths = find_lengths(&galaxies, expansion_lines, 2);
    lengths.iter().sum()
}

fn solve_part_b(input: &Vec<String>, coefficient: u64) -> u64 {
    let map = parse_input(input);
    let expansion_lines = find_expansion_lines(&map);
    let galaxies = find_galaxies(&map);
    let lengths = find_lengths(&galaxies, expansion_lines, coefficient);
    lengths.iter().sum()
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: u64 = solve_part_a(&input);
    let result_part_b: u64 = solve_part_b(&input, 1_000_000);
    println!("Part A result: {}", result_part_a);
    println!("Part B result: {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(374, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example1() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(1030, solve_part_b(&example, 10));
    }
    #[test]
    fn check_part_b_example2() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(8410, solve_part_b(&example, 100));
    }
}
