use rand::prelude::*;
use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
fn parse_input(input: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    input.iter().for_each(|line| {
        let mut iter = line.split(":");
        let node = iter.next().unwrap();
        let mut next_nodes = iter
            .next()
            .unwrap()
            .split(" ")
            .map(|node| node.trim().to_string())
            .collect::<Vec<String>>();
        next_nodes = next_nodes[1..].to_vec();
        graph.insert(node.to_string(), next_nodes.clone());
    });
    for (node, next_nodes) in &graph.clone() {
        for next_node in next_nodes {
            graph
                .entry(next_node.clone())
                .and_modify(|h| {
                    if !h.contains(&node.to_string()) {
                        h.push(node.to_string());
                    }
                })
                .or_insert(vec![node.to_string()]);
        }
    }
    graph
}

fn karger(graph: HashMap<String, Vec<String>>) -> u64 {
    let mut rng = rand::thread_rng();
    loop {
        // run iterations till we get a solution
        let mut temp_graph = graph.clone();
        // coalesce nodes till there are 2 left
        while temp_graph.len() > 2 {
            // get a random vertex and remove it from the graph
            let random_vertex = rng.gen_range(0..temp_graph.len());
            let old_v_1 = temp_graph.keys().nth(random_vertex).unwrap().clone();
            let mut neighbors = temp_graph.remove(&old_v_1).unwrap();

            // get a random neighbor of that vertex and remove it from the graph
            let random_neighbor = rng.gen_range(0..neighbors.len());
            let old_v_2 = neighbors.get(random_neighbor).unwrap().clone();
            let other_neighbors = temp_graph.remove(&old_v_2).unwrap();

            // create a new node with a naming convention that we can later unpack
            let coalesce = old_v_1.clone() + " " + &old_v_2;

            // append the two lists of connected vertices
            neighbors.extend(other_neighbors);

            // delete self links
            neighbors.retain(|vertex: &String| *vertex != old_v_1 && *vertex != old_v_2);

            // add node in the graph
            temp_graph.insert(coalesce.clone(), neighbors);

            // preserve the number of the connected vertices per vertex
            // (thats what we're looking for as an end goal)
            for (_, connected) in temp_graph.iter_mut() {
                for edge in connected {
                    if edge == &old_v_1 || edge == &old_v_2 {
                        *edge = coalesce.clone();
                    }
                }
            }
        }
        // if our graph has exactly three bridges we stop
        // and by our naming convetion we also know the number of each graph
        if temp_graph.values().last().unwrap().len() == 3 {
            return temp_graph
                .iter()
                .map(|(vertex, _)| vertex.split(" ").collect::<Vec<&str>>().len() as u64)
                .product::<u64>();
        }
    }
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let graph = parse_input(input);
    karger(graph)
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: u64 = solve_part_a(&input);
    println!("Part A result: {}", result_part_a);
    println!("Part B -> Press the button");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(54, solve_part_a(&example));
    }
}
