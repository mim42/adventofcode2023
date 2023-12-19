use std::{collections::HashMap, fs::read_to_string, ops::Range};

struct Workflow {
    conditions: Vec<Condition>,
}

struct Condition {
    category: Category,
    operator: Operator,
    number: u64,
    workflow: String,
}

enum Operator {
    More,
    Less,
    None,
}
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Category {
    X,
    M,
    A,
    S,
    None,
}

struct Rating {
    ratings: HashMap<Category, u64>,
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}
fn parse_input(input: &Vec<String>) -> (HashMap<String, Workflow>, Vec<Rating>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut ratings: Vec<Rating> = Vec::new();
    workflows.insert("A".to_string(), Workflow { conditions: vec![] });
    workflows.insert("R".to_string(), Workflow { conditions: vec![] });
    let mut flag = true;
    for line in input {
        if line == "" {
            flag = false;
            continue;
        }
        if flag {
            let a = line.find("{").unwrap();
            let name = line[..a].to_string();
            let mut conditions: Vec<Condition> = Vec::new();
            let c = line[a + 1..line.len() - 1].to_string();
            for con in c.split(",") {
                match con.find(":") {
                    Some(k) => conditions.push(Condition {
                        category: match &con[..1] {
                            "x" => Category::X,
                            "m" => Category::M,
                            "a" => Category::A,
                            "s" => Category::S,
                            _ => Category::None,
                        },
                        operator: match &con[1..2] {
                            ">" => Operator::More,
                            "<" => Operator::Less,
                            _ => Operator::None,
                        },
                        number: con[2..k].parse::<u64>().unwrap(),
                        workflow: con[k + 1..].to_string(),
                    }),
                    None => conditions.push(Condition {
                        category: Category::None,
                        operator: Operator::None,
                        number: 0,
                        workflow: con.to_string(),
                    }),
                }
            }
            let workflow = Workflow { conditions };
            workflows.insert(name, workflow);
        } else {
            let mut categories: HashMap<Category, u64> = HashMap::new();
            for s in line[1..line.len() - 1].split(",") {
                categories.insert(
                    match &s[..1] {
                        "x" => Category::X,
                        "m" => Category::M,
                        "a" => Category::A,
                        "s" => Category::S,
                        _ => Category::None,
                    },
                    s[2..].parse::<u64>().unwrap(),
                );
            }
            ratings.push(Rating {
                ratings: categories,
            });
        }
    }

    (workflows, ratings)
}

fn calculate_configurations(
    workflows: &HashMap<String, Workflow>,
    name: String,
    mut ranges: HashMap<Category, Range<u64>>,
) -> u64 {
    let mut result = 0;
    if name == "A" {
        return ranges
            .values()
            .fold(1, |acc, v| acc * (v.end - v.start + 1));
    } else if name == "R" {
        return 0;
    }

    let current = workflows.get(&name).unwrap();
    for condition in &current.conditions {
        match condition.operator {
            Operator::Less => {
                let mut ranges_new = ranges.clone();
                let range = ranges.get_mut(&condition.category).unwrap();
                let max = range.end;
                let min = range.start;

                if range.contains(&condition.number) {
                    *range = condition.number..max;
                    ranges_new.insert(condition.category.clone(), min..condition.number - 1);
                    result +=
                        calculate_configurations(workflows, condition.workflow.clone(), ranges_new);
                }
            }
            Operator::More => {
                let mut ranges_new = ranges.clone();
                let range = ranges.get_mut(&condition.category).unwrap();

                let max = range.end;
                let min = range.start;
                if range.contains(&condition.number) {
                    *range = min..condition.number;
                    ranges_new.insert(condition.category.clone(), condition.number + 1..max);
                    result +=
                        calculate_configurations(workflows, condition.workflow.clone(), ranges_new);
                }
            }
            Operator::None => {
                result +=
                    calculate_configurations(workflows, condition.workflow.clone(), ranges.clone());
            }
        }
    }

    result
}
fn solve_part_a(input: &Vec<String>) -> u64 {
    let (workflows, ratings) = parse_input(input);
    let starting_name = "in";
    let mut result = 0;
    for rating in ratings {
        let mut current_name = starting_name;
        while current_name != "A" && current_name != "R" {
            let current = workflows.get(current_name).unwrap();
            for condition in &current.conditions {
                match condition.operator {
                    Operator::Less => {
                        if *rating.ratings.get(&condition.category).unwrap() < condition.number {
                            current_name = &condition.workflow;
                            break;
                        }
                    }
                    Operator::More => {
                        if *rating.ratings.get(&condition.category).unwrap() > condition.number {
                            current_name = &condition.workflow;
                            break;
                        }
                    }
                    Operator::None => {
                        current_name = &condition.workflow;
                    }
                }
            }
        }
        if current_name == "A" {
            result += rating.ratings.values().sum::<u64>();
        }
    }

    result
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let (workflow, _) = parse_input(input);
    let mut ranges: HashMap<Category, Range<u64>> = HashMap::new();
    ranges.insert(Category::X, 1..4000);
    ranges.insert(Category::M, 1..4000);
    ranges.insert(Category::A, 1..4000);
    ranges.insert(Category::S, 1..4000);
    calculate_configurations(&workflow, "in".to_string(), ranges)
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
        assert_eq!(19114, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example.txt");
        assert_eq!(167409079868000, solve_part_b(&example));
    }
}
