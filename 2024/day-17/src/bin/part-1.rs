use std::ops::BitXor;

use itertools::Itertools;

struct State {
    a: u64,
    b: u64,
    c: u64,

    pp: usize,
}

type Program = Vec<u64>;

fn parse(input: &str) -> (State, Program) {
    let (state, program) = input.trim().split_once("\n\n").unwrap();
    let mut state_lines = state.lines();
    let a = state_lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let b = state_lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let c = state_lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let program: Program = program[9..]
        .split(',')
        .map(|c| c.parse::<u64>().unwrap())
        .collect();

    (State { a, b, c, pp: 0 }, program)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Output {
    Halt,
    Out(u64),
    Run,
}

fn combo(state: &State, operand: u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        op => panic!("incorrect combo operand: {op}"),
    }
}

fn run(state: &mut State, program: &Program) -> Output {
    if state.pp >= program.len() {
        return Output::Halt;
    }
    let op = program[state.pp];
    state.pp += 1;
    match op {
        0 => {
            let op2 = program[state.pp];
            state.a /= 2u64.pow(combo(state, op2) as u32);
            state.pp += 1;
            Output::Run
        }
        1 => {
            let op2 = program[state.pp];
            state.b = state.b.bitxor(op2);
            state.pp += 1;
            Output::Run
        }
        2 => {
            let op2 = program[state.pp];
            state.b = combo(state, op2) % 8;
            state.pp += 1;
            Output::Run
        }
        3 => {
            let op2 = program[state.pp];
            state.pp += 1;
            if state.a != 0 {
                state.pp = op2 as usize;
            }
            Output::Run
        }
        4 => {
            let _op2 = program[state.pp];
            state.pp += 1;
            state.b = state.b.bitxor(state.c);
            Output::Run
        }
        5 => {
            let op2 = program[state.pp];
            state.pp += 1;
            Output::Out(combo(state, op2) % 8)
        }
        6 => {
            let op2 = program[state.pp];
            state.pp += 1;
            state.b = state.a / 2u64.pow(combo(state, op2) as u32);
            Output::Run
        }
        7 => {
            let op2 = program[state.pp];
            state.pp += 1;
            state.c = state.a / 2u64.pow(combo(state, op2) as u32);
            Output::Run
        }
        op => {
            panic!("incorrect opcode: {op}")
        }
    }
}

fn part_1(input: &str) -> String {
    let (mut state, program) = parse(input);
    let mut output: Vec<u64> = Vec::new();
    loop {
        match run(&mut state, &program) {
            Output::Halt => break,
            Output::Out(out) => output.push(out),
            Output::Run => {}
        }
    }

    output.iter().map(|n| n.to_string()).join(",")
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
        let input: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(part_1(input), "4,6,3,5,6,3,5,2,1,0");
    }
}
