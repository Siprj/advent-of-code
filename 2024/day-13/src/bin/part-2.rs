use ndarray::{array, Array1, Array2};
use ndarray_linalg::Solve;

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    target: (i64, i64),
}

fn parse(input: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = vec![];
    for machine in input.trim().split("\n\n") {
        let (a, rest) = machine.split_once('\n').unwrap();

        let (b, t) = rest.split_once('\n').unwrap();
        let (ax, ay) = a.split_once(',').unwrap();
        let (bx, by) = b.split_once(',').unwrap();
        let (tx, ty) = t.split_once(',').unwrap();
        let a = (ax[12..].parse().unwrap(), ay[3..].parse().unwrap());
        let b = (bx[12..].parse().unwrap(), by[3..].parse().unwrap());
        let t = (tx[9..].parse().unwrap(), ty[3..].parse().unwrap());
        machines.push(Machine { a, b, target: t });
    }
    machines
}

fn solve(machine: &Machine) -> Option<(i64, i64)> {
    let a: Array2<f64> = array![
        [machine.a.0 as f64, machine.b.0 as f64],
        [machine.a.1 as f64, machine.b.1 as f64]
    ];
    let b: Array1<f64> = array![machine.target.0 as f64, machine.target.1 as f64];
    match a.solve(&b) {
        Ok(ns) => {
            let n1 = ns[0].round() as i64;
            let n2 = ns[1].round() as i64;
            if n1 * machine.a.0 + n2 * machine.b.0 == machine.target.0
                && n1 * machine.a.1 + n2 * machine.b.1 == machine.target.1
            {
                Some((n1, n2))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn part_2(input: &str) -> String {
    let mut machines = parse(input);

    for machine in machines.iter_mut() {
        machine.target.0 += 10000000000000;
        machine.target.1 += 10000000000000;
    }
    let mut sum = 0;
    for machine in machines.iter() {
        if let Some((n1, n2)) = solve(machine) {
            sum += n1 * 3 + n2;
        }
    }

    sum.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let len = input.len();
    let result = part_2(&input[..len - 1]);
    println!("Part 2: {}", result);
}
