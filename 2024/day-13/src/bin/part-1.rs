use ndarray::{array, Array1, Array2};
use ndarray_linalg::Solve;

#[derive(Debug)]
struct Machine {
    a: (i32, i32),
    b: (i32, i32),
    target: (i32, i32),
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

fn solve(machine: &Machine) -> Option<(i32, i32)> {
    let a: Array2<f64> = array![
        [machine.a.0 as f64, machine.b.0 as f64],
        [machine.a.1 as f64, machine.b.1 as f64]
    ];
    let b: Array1<f64> = array![machine.target.0 as f64, machine.target.1 as f64];
    match a.solve(&b) {
        Ok(ns) => {
            let n1 = ns[0].round() as i32;
            let n2 = ns[1].round() as i32;
            if n1 * machine.a.0 + n2 * machine.b.0 == machine.target.0
                && n1 * machine.a.1 + n2 * machine.b.1 == machine.target.1
                && n1 <= 100
                && n2 <= 100
            {
                Some((n1, n2))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn part_1(input: &str) -> String {
    let machines = parse(input);

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
    let result = part_1(&input[..len - 1]);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(part_1(input), "480");
    }
}
