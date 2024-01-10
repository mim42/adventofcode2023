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
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

// simple dfs into our constructed graph with important vertices
fn find_longest(
    point: &Point,
    end_point: &Point,
    vertices: &HashMap<Point, HashMap<Point, u64>>,
    visited: &mut Vec<Point>,
    length: u64,
) -> u64 {
    if point == end_point {
        return length;
    }
    let next_points = vertices.get(point).unwrap();
    let mut max_value = 0;
    for (k, v) in next_points {
        if !visited.contains(k) {
            visited.push(*k);
            let result = find_longest(k, end_point, vertices, visited, length + *v);
            visited.pop();
            if result > max_value {
                max_value = result;
            }
        }
    }
    max_value
}

fn populate_vertices(
    vertex: Point,
    map: &Vec<Vec<String>>,
    vertices: &mut HashMap<Point, HashMap<Point, u64>>,
) {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut next_steps: Vec<(Point, u64)> = Vec::new();
    next_steps.push((vertex.clone(), 0));
    let steps: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // we bfs into the maze till as we find other important vertice and we mark it with the length from our initial vertex
    while !next_steps.is_empty() {
        let (current, length) = next_steps.remove(0);
        visited.insert(current);
        if vertices.contains_key(&current) && current != vertex {
            let v = vertices.get_mut(&vertex).unwrap();
            (*v).insert(current, length);
        } else {
            for dir in 0..4 {
                let (step_x, step_y) = steps[dir];
                let next_x = current.x as i32 + step_x;
                let next_y = current.y as i32 + step_y;
                if next_x >= (map.len() as i32)
                    || next_x < 0
                    || next_y >= (map[0].len() as i32)
                    || next_y < 0
                {
                    continue;
                }

                match map[next_x as usize][next_y as usize].as_str() {
                    "#" => (),
                    _ => {
                        let next_point = Point {
                            x: next_x as usize,
                            y: next_y as usize,
                        };
                        if !visited.contains(&next_point) {
                            next_steps.push((next_point, length + 1))
                        }
                    }
                }
            }
        }
    }
}

fn build_vertices(map: &Vec<Vec<String>>) -> HashMap<Point, HashMap<Point, u64>> {
    let mut vertices: HashMap<Point, HashMap<Point, u64>> = HashMap::new();
    vertices.insert(Point { x: 0, y: 1 }, HashMap::new());
    vertices.insert(
        Point {
            x: map.len() - 1,
            y: map[0].len() - 2,
        },
        HashMap::new(),
    );
    // find all vertices of importance, (ie where we split ways)
    for x in 1..map.len() - 1 {
        for y in 1..map[0].len() - 1 {
            if map[x][y] == "#" {
                continue;
            }
            let steps: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            let mut counter = 0;
            for dir in 0..4 {
                let (step_x, step_y) = steps[dir];
                let next_x = x as i32 + step_x;
                let next_y = y as i32 + step_y;

                match map[next_x as usize][next_y as usize].as_str() {
                    "#" => (),
                    _ => counter += 1,
                }
            }
            if counter >= 3 {
                vertices.insert(Point { x, y }, HashMap::new());
            }
        }
    }
    // for every vertex build its hashmap of connected vertices and their distance from them.
    let mut id = 0;
    for (i, _) in vertices.clone() {
        populate_vertices(i, map, &mut vertices);
    }
    vertices
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let map = parse_input(input);
    let vertices = build_vertices(&map);

    find_longest(
        &Point { x: 0, y: 1 },
        &(Point {
            x: map.len() - 1,
            y: map[0].len() - 2,
        }),
        &vertices,
        &mut Vec::new(),
        0,
    )
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_b: u64 = solve_part_b(&input);
    println!("Part B result: {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(154, solve_part_b(&example));
    }
}
