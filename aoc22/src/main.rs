use std::{collections::HashMap, fs::read_to_string, ops::Range};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
fn parse_input(input: &Vec<String>) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();
    for i in input {
        let split = i.split("~").collect::<Vec<&str>>();
        let start = split[0]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let end = split[1]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        bricks.push(Brick {
            x: start[0]..end[0] + 1,
            y: start[1]..end[1] + 1,
            z: start[2]..end[2] + 1,
        })
    }
    bricks
}
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Brick {
    x: Range<usize>,
    y: Range<usize>,
    z: Range<usize>,
}
fn collides(a: &Brick, b: &Brick) -> bool {
    a.x.clone().any(|i| b.x.contains(&i)) && a.y.clone().any(|i| b.y.contains(&i))
}

fn simulation_a(bricks: &Vec<Brick>) -> (HashMap<Brick, Vec<Brick>>, HashMap<Brick, Vec<Brick>>) {
    let mut stack: Vec<Brick> = Vec::new();
    let mut below: HashMap<Brick, Vec<Brick>> = HashMap::new();
    let mut above: HashMap<Brick, Vec<Brick>> = HashMap::new();

    for input_brick in bricks {
        let result = stack
            .iter()
            .filter(|brick| collides(&input_brick, brick))
            .max_by_key(|brick| brick.z.end);
        let mut fallen_brick = input_brick.clone();
        match result {
            Some(next_brick) => {
                let diff = fallen_brick.z.end - fallen_brick.z.start;
                fallen_brick.z.start = next_brick.z.end;
                fallen_brick.z.end = fallen_brick.z.start + diff;
            }
            None => {
                let diff = fallen_brick.z.start;
                fallen_brick.z.start = 1;
                fallen_brick.z.end = fallen_brick.z.end - diff + 1;
            }
        }
        // build dependency tree from top to bottom
        let supporting_bricks = stack
            .iter()
            .filter(|brick| collides(&fallen_brick, brick) && brick.z.end == fallen_brick.z.start)
            .map(|b| b.clone())
            .collect::<Vec<Brick>>();
        below.insert(fallen_brick.clone(), supporting_bricks);
        stack.push(fallen_brick);
    }

    //Build dependency tree from bottom up
    for brick in &stack {
        above.insert(brick.clone(), vec![]);
        for (b, vec) in &below {
            if vec.contains(brick) {
                (*above.get_mut(brick).unwrap()).push(b.clone());
            }
        }
    }
    (below, above)
}

fn simulation_b(disintegrated: Option<&Brick>, bricks: &Vec<Brick>) -> HashMap<u64, Brick> {
    let mut stack: HashMap<u64, Brick> = HashMap::new();
    for (index, input_brick) in bricks.iter().enumerate() {
        match disintegrated {
            Some(v) => {
                if v == input_brick {
                    continue;
                }
            }
            None => (),
        }
        let result = stack
            .iter()
            .filter(|(_, brick)| collides(&input_brick, brick))
            .max_by_key(|(_, brick)| brick.z.end);
        let mut fallen_brick = input_brick.clone();
        match result {
            Some((_, next_brick)) => {
                let diff = fallen_brick.z.end - fallen_brick.z.start;
                fallen_brick.z.start = next_brick.z.end;
                fallen_brick.z.end = fallen_brick.z.start + diff;
            }
            None => {
                let diff = fallen_brick.z.start;
                fallen_brick.z.start = 1;
                fallen_brick.z.end = fallen_brick.z.end - diff + 1;
            }
        }
        stack.insert(index as u64, fallen_brick);
    }
    stack
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let mut bricks = parse_input(input);
    bricks.sort_by_key(|c| c.z.start);
    let (below, above) = simulation_a(&bricks);
    let mut answer = 0;
    //for every cube that is supported by the current one, if they rely on at least 2 cubes
    // we can disintegrate the current one
    for (_, supported) in &above {
        if supported.len() == 0 {
            answer += 1;
        } else {
            if supported
                .iter()
                .all(|c: &Brick| below.get(c).unwrap().len() > 1)
            {
                answer += 1;
            }
        }
    }
    answer as u64
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let mut bricks = parse_input(input);
    bricks.sort_by_key(|c| c.z.start);
    let stack: HashMap<u64, Brick> = simulation_b(None, &bricks);
    let mut answer = 0;
    // I tried a more elaborate solution but i like this approach better.
    // I ran the initial simulation for all the bricks except one and I count the differences
    // from the original stack

    for disintegrated in &bricks {
        for (k, v) in &simulation_b(Some(disintegrated), &bricks) {
            if v != stack.get(&k).unwrap() {
                answer += 1;
            }
        }
    }
    answer
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
        assert_eq!(5, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(7, solve_part_b(&example));
    }
}
