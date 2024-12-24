use core::panic;
use std::collections::HashMap;

enum Logic {
    And,
    Or,
    Xor,
}

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

fn solve<'a>(
    values: &mut HashMap<&'a str, bool>,
    operations: &HashMap<&'a str, Operation<'a>>,
    output: &'a str,
) -> bool {
    let op = operations.get(output).unwrap();
    let input1 = if let Some(v) = values.get(op.input1) {
        *v
    } else {
        solve(values, operations, op.input1)
    };
    let input2 = if let Some(v) = values.get(op.input2) {
        *v
    } else {
        solve(values, operations, op.input2)
    };
    let out = match op.operation {
        Logic::And => input1 && input2,
        Logic::Or => input1 || input2,
        Logic::Xor => input1 ^ input2,
    };
    values.insert(output, out);
    out
}

fn part_1(input: &str) -> String {
    let (initail_values, operations) = parse(input);
    let mut values: HashMap<&str, bool> = HashMap::from_iter(initail_values.iter().copied());
    let outputs: Vec<&str> = operations
        .iter()
        .filter_map(|op| {
            if op.output.starts_with("z") {
                Some(op.output)
            } else {
                None
            }
        })
        .collect();
    let operations: HashMap<&str, Operation> =
        operations.into_iter().map(|op| (op.output, op)).collect();

    let outputs: Vec<(&str, bool)> = outputs
        .into_iter()
        .map(|output| (output, solve(&mut values, &operations, output)))
        .collect();
    let mut ret = 0;

    for (name, value) in outputs.iter() {
        let index: usize = name[1..].parse().unwrap();
        ret += (*value as usize) << index;
    }

    ret.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!(part_1(input), "4");
    }

    #[test]
    fn it_works2() {
        let input: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!(part_1(input), "2024");
    }
}
