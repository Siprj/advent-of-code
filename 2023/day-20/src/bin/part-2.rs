use std::collections::{HashMap, HashSet, VecDeque};

use num::integer::lcm;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Puls {
    High, Low
}

impl Puls {
    fn flip(&mut self) -> Puls{
        match self {
            Puls::High => *self = Puls::Low,
            Puls::Low => *self = Puls::High,
        }
        *self
    }
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
}

fn parse(input: &str) -> HashMap<String, Module>{
    let mut modules = HashMap::new();

    for l in input.lines() {
        let (left, right) = l.split_once(" -> ").unwrap();
        use ModuleType::*;
        let (module_type, module_name) = match left.as_bytes()[0] {
            b'&' => (Conjunction, left[1..].to_string()),
            b'%' => (FlipFlop, left[1..].to_string()),
            _ => (Broadcaster, left.to_string()),
        };
        let destinations = right.split(", ").map(|part| part.to_string()).collect();
        modules.insert(module_name, Module{module_type, destinations});
    }
    modules
}

#[derive(Debug, Clone)]
enum ModuleTypeState {
    FlipFlop(Puls),
    Conjunction(HashMap<String, Puls>),
    Broadcaster,
}

#[derive(Debug, Clone)]
struct ModuleState {
    module_type: ModuleTypeState,
    destinations: Vec<String>,
}

fn initialize_state(modules: &HashMap<String,Module>) -> HashMap<String, ModuleState> {
    let module_inputs: HashMap<String, HashSet<String>> = modules.iter().fold(HashMap::new(), |mut acc, module| {
        for destination in module.1.destinations.iter() {
            acc.entry(destination.clone()).or_default().insert(module.0.clone());
        }
        acc
    });
    modules.iter().map(|(name, module)| {
        let module_type: ModuleTypeState = match module.module_type {
            ModuleType::FlipFlop => ModuleTypeState::FlipFlop(Puls::Low),
            ModuleType::Conjunction => ModuleTypeState::Conjunction(module_inputs.get(name).unwrap().iter().map(|v| (v.clone(), Puls::Low)).collect()),
            ModuleType::Broadcaster => ModuleTypeState::Broadcaster,
        };
        (name.clone(), ModuleState {module_type, destinations: module.destinations.clone()})
    }).collect()
}

fn solve(mut modules: HashMap<String, ModuleState>, looking_for: &str) -> u64 {
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for i in 1.. {
        if i % 100000 == 0{
            println!("i: {}", i);
        }
        let mut pulses_to_process: VecDeque<(String, Puls, String)> = VecDeque::new();
        pulses_to_process.push_back(("broadcaster".to_string(), Puls::Low, "button".to_string()));
        low_pulses += 1;
        while let Some((to, puls, from)) = pulses_to_process.pop_front() {
            if looking_for == to && puls == Puls::Low {
                    return i;
            }
            if let Some(destination_module) = modules.get_mut(&to) {
                let empty_vec = vec![];
                let (puls, destinations): (Puls, &[String]) = match &mut destination_module.module_type {
                    ModuleTypeState::FlipFlop(flip_flop_state) => {
                        if puls == Puls::Low {
                            let new_puls = flip_flop_state.flip();
                            (new_puls, &destination_module.destinations)
                        } else {
                            (Puls::Low, &empty_vec)
                        }
                    },
                    ModuleTypeState::Conjunction(conjunction_state) => {
                        *conjunction_state.get_mut(&from).unwrap() = puls;
                        if conjunction_state.iter().all(|v| v.1 == &Puls::High) {
                            (Puls::Low, &destination_module.destinations)
                        } else {
                            (Puls::High, &destination_module.destinations)
                        }
                    },
                    ModuleTypeState::Broadcaster => (puls, &destination_module.destinations),
                };
                for destination in destinations {
                    match puls {
                        Puls::High => high_pulses += 1,
                        Puls::Low => low_pulses += 1,
                    }
                    pulses_to_process.push_back((destination.clone(), puls, to.clone()));
                }
            }
        }
    }
    low_pulses * high_pulses
}

fn generate_dot(modules: &HashMap<String, Module>) -> String {
    let mut output = String::new();
    output.push_str("digraph g {\nbutton -> broadcaster;\n");
    for (name, module) in modules.iter() {
        for destination_name in module.destinations.clone() {
            let mut to = String::new();
            if let Some(destination_module) = modules.get(&destination_name) {
                to.push('\"');
                match destination_module.module_type {
                    ModuleType::FlipFlop => to.push_str("\\%"),
                    ModuleType::Conjunction => to.push('&'),
                    ModuleType::Broadcaster => {},
                }
                to.push_str(&destination_name);
                to.push('\"');
            } else {
                to.push_str(&destination_name);
            }

            let mut from = String::new();
            from.push('\"');
            match module.module_type {
                ModuleType::FlipFlop => from.push_str("\\%"),
                ModuleType::Conjunction => from.push('&'),
                ModuleType::Broadcaster => {},
            }
            from.push_str(name);
            from.push('\"');

            output.push_str(&from);
            output.push_str(" -> ");
            output.push_str(&to);
            output.push_str(";\n");

        }

    }
    output.push_str("}\n");
    output
}

fn part_2(input: &str) -> String {
    let modules = parse(input);
    let dot = generate_dot(&modules);
    // This solution is done based on the data analysis, not sure what the
    // general solution would be. I guess One could walk backwards and try to
    // find the loops by test running the solve algorithm on the sub graphs.
    println!("{dot}");
    let mut output = 1;
    for looking_for in ["hf", "sb", "nd", "ds"] {
        let initialized_modules = initialize_state(&modules);
        let m = solve(initialized_modules, looking_for);
        output = lcm(output, m);
    }
    output.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);
    println!("Result: {}", result);
}
