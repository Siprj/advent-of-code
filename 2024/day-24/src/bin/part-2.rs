use core::panic;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
enum Logic {
    And,
    Or,
    Xor,
}

impl Display for Logic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Logic::And => "AND",
            Logic::Or => "OR",
            Logic::Xor => "XOR",
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug)]
struct Operation<'a> {
    input1: &'a str,
    input2: &'a str,
    operation: Logic,
    output: &'a str,
}

fn parse(input: &str) -> (Vec<(&str, bool)>, Vec<Operation>) {
    let (values, operations) = input.trim().split_once("\n\n").unwrap();
    let values: Vec<(&str, bool)> = values
        .lines()
        .map(|l| {
            let (name, value) = l.split_once(": ").unwrap();
            (name, value.starts_with('1'))
        })
        .collect();
    let operations: Vec<Operation> = operations
        .lines()
        .map(|l| {
            let l: Vec<&str> = l.split_whitespace().collect();
            let op = match l[1] {
                "AND" => Logic::And,
                "OR" => Logic::Or,
                "XOR" => Logic::Xor,
                _ => {
                    panic!("not a valid operation");
                }
            };
            Operation {
                input1: l[0],
                input2: l[2],
                operation: op,
                output: l[4],
            }
        })
        .collect();

    (values, operations)
}

fn part_2(input: &str) -> String {
    let (initail_values, operations) = parse(input);
    println!("digraph graphname {{");

    let input_len = initail_values.len() /2;

    let mut swapped: HashSet<String> = HashSet::new();

    println!("{operations:?}");

    let z00 = operations
        .iter()
        .find(|op| op.input1 == "y00" && op.input2 == "x00" && op.operation == Logic::Xor)
        .unwrap();

    if z00.output != "z00" {
        swapped.insert(z00.output.to_string());
    }

    let mut carry: &str = operations
        .iter()
        .find_map(|op| {
            if op.input1 == "x00" && op.input2 == "y00" && op.operation == Logic::And {
                Some(op.output)
            } else {
                None
            }
        })
        .unwrap();

    for index in 1..input_len {
        let x = format!("x{index:02}");
        let y = format!("y{index:02}");
        let z = format!("z{index:02}");
        let basic_add = operations
            .iter()
            .find(|op| ((op.input1 == y && op.input2 == x) || (op.input1 == x && op.input2 == y)) && op.operation == Logic::Xor)
            .unwrap()
            .output;

        let add = operations
            .iter()
            .find(|op| op.operation == Logic::Xor && (op.input1 == carry || op.input2 == carry || op.input1 == basic_add || op.input2 == basic_add))
            .unwrap();
        if add.output != z {
            swapped.insert(z);
            swapped.insert(add.output.to_string());
        }

        if !(add.input1 == basic_add || add.input2 == basic_add) {
            swapped.insert(basic_add.to_string());
        }

        if !(add.input1 == carry || add.input2 == carry) {
            swapped.insert(carry.to_string());
        }
        let basic_carry = operations
            .iter()
            .find(|op| ((op.input1 == x && op.input2 == y) || (op.input1 == y && op.input2 == x)) && op.operation == Logic::And)
            .unwrap()
            .output;
        let cascade_carry = operations
            .iter()
            .find(|op| op.operation == Logic::And && (op.input1 == basic_add || op.input2 == basic_add || op.input1 == carry || op.input2 == carry))
            .unwrap();

        if !(cascade_carry.input1 == basic_add || cascade_carry.input2 == basic_add) {
            swapped.insert(basic_add.to_string());
        }

        if !(cascade_carry.input1  == carry || cascade_carry.input2 == carry) {
            swapped.contains(carry);
        }
        let carry_gate = operations
            .iter()
            .find(|op| {
                op.operation == Logic::Or
                    && (op.input1 == cascade_carry.output || op.input2 == cascade_carry.output || op.input1 == basic_carry || op.input2 == basic_carry)
            })
            .unwrap();

        if !(carry_gate.input1 == cascade_carry.output || carry_gate.input2 == cascade_carry.output) {
            swapped.insert(cascade_carry.output.to_string());
        }

        if !(carry_gate.input1 == basic_carry || carry_gate.input2 == basic_carry) {
            swapped.insert(basic_carry.to_string());
        }

        carry = carry_gate.output;
    }

    let mut vec: Vec<String> = swapped.into_iter().collect();
    vec.sort();
    vec.join(",")
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);
    println!("Part 2: {}", result);
}
