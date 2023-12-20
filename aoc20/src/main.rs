use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone)]
struct Module {
    t: Type,
    connected: Vec<String>,
    memory: HashMap<String, Pulse>,
    state: State,
}
#[derive(Clone, Debug, PartialEq)]
enum Type {
    Broadcaster,
    FlipFlop,
    Conjunction,
    None,
}
#[derive(Hash, Clone, Debug, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}
#[derive(Clone, Debug)]
enum State {
    On,
    Off,
}
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_input(input: &Vec<String>) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in input {
        let a: usize = line.find(" -").unwrap();
        let mut name = line[..a].to_string();
        let t: Type;
        match name.as_str() {
            "broadcaster" => t = Type::Broadcaster,
            _ => match name.find("&") {
                Some(_) => {
                    name = name[1..].to_string();
                    t = Type::Conjunction;
                }
                None => {
                    name = name[1..].to_string();
                    t = Type::FlipFlop;
                }
            },
        }
        let c = line[a + 3..].to_string();
        let mut connected: Vec<String> = Vec::new();
        let memory: HashMap<String, Pulse> = HashMap::new();
        for con in c.split(",") {
            connected.push(con.trim().to_string());
        }
        modules.insert(
            name,
            Module {
                t,
                connected,
                memory,
                state: State::Off,
            },
        );
    }
    modules
}

// Populate the memory field of all the conjunctions. As the input is not ideal to do it while parsing.
// Also create modules that are only present as end states. Like output or rx
fn build_memory(modules: &mut HashMap<String, Module>) {
    for (name, module) in modules.clone() {
        for i in &module.connected {
            match modules.get_mut(i) {
                Some(m) => match m.t {
                    Type::Conjunction => {
                        m.memory.insert(name.clone(), Pulse::Low);
                    }
                    Type::FlipFlop => (),
                    _ => unreachable!(),
                },
                None => {
                    modules.insert(
                        i.to_string(),
                        Module {
                            t: Type::None,
                            memory: HashMap::new(),
                            connected: Vec::new(),
                            state: State::Off,
                        },
                    );
                }
            };
        }
    }
}

fn send_pulse(modules: &mut HashMap<String, Module>) -> (u64, u64) {
    let mut counter: HashMap<Pulse, u64> = HashMap::new();
    counter.insert(Pulse::Low, 0);
    counter.insert(Pulse::High, 0);

    for _ in 0..1000 {
        *counter.entry(Pulse::Low).or_default() += 1;
        let mut queue: Vec<(String, Pulse, String)> = Vec::new();
        queue.push(("broadcaster".to_string(), Pulse::Low, "button".to_string()));

        while !queue.is_empty() {
            let (module_name, input_pulse, prev_module) = queue.remove(0);
            let module = modules.get_mut(&module_name).unwrap();
            match module.t {
                Type::Broadcaster => {
                    for i in &module.connected {
                        *counter.entry(Pulse::Low).or_default() += 1;
                        queue.push((i.to_string(), Pulse::Low, module_name.clone()));
                    }
                }
                Type::Conjunction => {
                    module.memory.insert(prev_module, input_pulse.clone());
                    let next_pulse: Pulse;
                    if module.memory.iter().all(|(_, p)| *p == Pulse::High) {
                        next_pulse = Pulse::Low;
                    } else {
                        next_pulse = Pulse::High;
                    }
                    for i in &module.connected {
                        *counter.entry(next_pulse.clone()).or_default() += 1;
                        queue.push((i.to_string(), next_pulse.clone(), module_name.clone()));
                    }
                }
                Type::FlipFlop => match input_pulse {
                    Pulse::High => (),
                    Pulse::Low => {
                        let next_pulse: Pulse;
                        match module.state {
                            State::Off => {
                                module.state = State::On;
                                next_pulse = Pulse::High
                            }
                            State::On => {
                                module.state = State::Off;
                                next_pulse = Pulse::Low
                            }
                        }

                        for i in &module.connected {
                            *counter.entry(next_pulse.clone()).or_default() += 1;
                            queue.push((i.to_string(), next_pulse.clone(), module_name.clone()));
                        }
                    }
                },
                Type::None => (),
            }
        }
    }

    (
        *counter.get(&Pulse::Low).unwrap(),
        *counter.get(&Pulse::High).unwrap(),
    )
}

fn find_rx_low_cycle(modules: &mut HashMap<String, Module>) -> Vec<u64> {
    let mut counter = 0;
    let mut prev = modules
        .get("dn")
        .unwrap()
        .memory
        .iter()
        .map(|(name, _)| (name.clone(), 0))
        .collect::<HashMap<String, u64>>();

    loop {
        counter += 1;
        if !prev.iter().any(|(_, c)| *c == 0) {
            // I have found all cycles for all previous states of dn with a high pulse
            // (which will result in a low rx)
            break;
        }
        let mut queue: Vec<(String, Pulse, String)> = Vec::new();
        queue.push(("broadcaster".to_string(), Pulse::Low, "button".to_string()));
        while !queue.is_empty() {
            let (module_name, input_pulse, prev_module) = queue.remove(0);
            let module = modules.get_mut(&module_name.clone()).unwrap();
            match module.t {
                Type::Broadcaster => {
                    for i in &module.connected {
                        queue.push((i.to_string(), Pulse::Low, module_name.clone()));
                    }
                }
                Type::Conjunction => {
                    module.memory.insert(prev_module, input_pulse.clone());
                    let next_pulse: Pulse;
                    if module.memory.iter().all(|(_, p)| *p == Pulse::High) {
                        next_pulse = Pulse::Low;
                    } else {
                        next_pulse = Pulse::High;
                    }
                    for i in &module.connected {
                        // The first moment we encounter dn in the connected states we insert
                        // the cycle in our hashmap
                        if i == "dn" && next_pulse == Pulse::High {
                            if *prev.get(&module_name).unwrap() == 0 {
                                prev.insert(module_name.clone(), counter);
                            }
                        }
                        queue.push((i.to_string(), next_pulse.clone(), module_name.clone()));
                    }
                }
                Type::FlipFlop => match input_pulse {
                    Pulse::High => (),
                    Pulse::Low => {
                        let next_pulse: Pulse;
                        match module.state {
                            State::Off => {
                                module.state = State::On;
                                next_pulse = Pulse::High
                            }
                            State::On => {
                                module.state = State::Off;
                                next_pulse = Pulse::Low
                            }
                        }

                        for i in &module.connected {
                            queue.push((i.to_string(), next_pulse.clone(), module_name.clone()));
                        }
                    }
                },
                Type::None => (),
            }
        }
    }
    prev.iter().map(|(_, c)| *c).collect::<Vec<u64>>()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(numbers: &[u64]) -> u64 {
    let a = numbers[0];
    let mut b = numbers[1];
    if numbers.len() > 2 {
        b = lcm(&numbers[1..]);
    }
    a * b / gcd(a, b)
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let mut modules = parse_input(input);
    build_memory(&mut modules);
    let (low, high) = send_pulse(&mut modules);
    low * high
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let mut modules = parse_input(input);
    build_memory(&mut modules);
    let cycles = find_rx_low_cycle(&mut modules);
    lcm(&cycles)
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
    fn check_part_a_example_1() {
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(32000000, solve_part_a(&example));
    }

    #[test]
    fn check_part_a_example_2() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(11687500, solve_part_a(&example));
    }
}
