use std::fs::read_to_string;

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
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

// simple dfs into our constructed graph with important vertices
fn find_longest(
    point_index: usize,
    end_index: usize,
    vertices: &[[(usize, u64); 4]; 64],
    visited: &mut [bool; 64],
    length: u64,
) -> u64 {
    if point_index == end_index {
        return length;
    }
    let mut max_value = 0;
    for (k, v) in vertices[point_index] {
        if v != 0 && !visited[point_index] {
            visited[point_index] = true;
            let result = find_longest(k, end_index, vertices, visited, length + v);
            visited[point_index] = false;
            if result > max_value {
                max_value = result;
            }
        }
    }
    max_value
}

fn populate_vertices(
    vertex_index: usize,
    map: &Vec<Vec<String>>,
    vertices: &mut [(Point, [(Point, u64); 4]); 64],
) {
    let mut visited: Vec<Point> = Vec::new();
    let mut next_steps: Vec<(Point, u64)> = Vec::new();
    next_steps.push((vertices[vertex_index].0, 0));
    let steps: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut neighbor_counter = 0;
    // we bfs into the maze till as we find other important vertice and we mark it with the length from our initial vertex
    while !next_steps.is_empty() {
        let (current, length) = next_steps.remove(0);
        visited.push(current.clone());

        if vertices.iter().any(|(e, _)| e == &current) && current != vertices[vertex_index].0 {
            vertices[vertex_index].1[neighbor_counter] = (current, length);
            neighbor_counter += 1;
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

fn build_vertices(map: &Vec<Vec<String>>) -> (usize, [[(usize, u64); 4]; 64]) {
    let mut vertices: [(Point, [(Point, u64); 4]); 64] =
        [(Point { x: 0, y: 0 }, [(Point { x: 0, y: 0 }, 0); 4]); 64];

    vertices[0] = (Point { x: 0, y: 1 }, [(Point { x: 0, y: 0 }, 0); 4]);

    // find all vertices of importance, (ie where we split ways)
    let mut index_counter: usize = 1;
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
                vertices[index_counter] = (Point { x, y }, [(Point { x: 0, y: 0 }, 0); 4]);
                index_counter += 1;
            }
        }
    }
    vertices[index_counter] = (
        Point {
            x: map.len() - 1,
            y: map[0].len() - 2,
        },
        [(Point { x: 0, y: 0 }, 0); 4],
    );

    // for every vertex find connected vertices and their distance from them.
    for i in 0..=index_counter {
        populate_vertices(i, map, &mut vertices);
    }
    // each index should be pointing to another index with a distance
    let mut new_vertices = [[(0, 0); 4]; 64];
    for i in 0..=index_counter {
        let (_, list) = vertices[i];
        for (index, (point, length)) in list.iter().filter(|&(_, v)| *v != 0).enumerate() {
            new_vertices[i][index] = (
                vertices.iter().position(|(p, _)| p == point).unwrap(),
                *length,
            );
        }
    }
    (index_counter, new_vertices)
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let map = parse_input(input);
    let (end_index, vertices) = build_vertices(&map);
    find_longest(0, end_index, &vertices, &mut [false; 64], 0)
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
