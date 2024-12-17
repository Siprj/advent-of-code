use std::{iter, ops::BitXor};

#[derive(Clone)]
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

fn exec(a: u64, mut state: State, program: &Program) -> Vec<u64> {
    state.a = a;
    let mut o: Vec<u64> = Vec::new();
    loop {
        match run(&mut state, program) {
            Output::Halt => break,
            Output::Out(out) => {
                o.push(out);
            }
            Output::Run => {}
        }
    }
    o
}

fn part_2(input: &str) -> String {
    let (state, program) = parse(input);
    let mut power: u32 = 15;
    let mut iterations: usize = 0;
    let mut diffs: Vec<u64> = iter::repeat(4).take(16).collect();
    diffs[15] = 8u64.pow(power);
    loop {
        iterations += 1;
        if iterations > 1000 {
            return "BORK2".to_string();
        }
        let low = diffs[(power as usize)..].iter().sum();
        let out: Vec<u64> = exec(low, state.clone(), &program);
        if let Some((i, _)) = out
            .iter()
            .enumerate()
            .rev()
            .find(|(i, &n)| program[*i] != n)
        {
            power = i as u32;
            diffs[i] += 8u64.pow(power);
            for d in diffs[0..i].iter_mut() {
                *d = 0;
            }
        } else {
            return low.to_string();
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(part_2(input), "117440");
    }
}
