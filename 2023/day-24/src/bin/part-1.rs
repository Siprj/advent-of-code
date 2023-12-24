use itertools::Itertools;

struct ParametricLine {
    x: f64,
    y: f64,
    z: f64,
    x_d: f64,
    y_d: f64,
    z_d: f64,
}

fn parse(input: &str) -> Vec<ParametricLine> {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(" @ ").unwrap();
            let (x, rest) = left.split_once(", ").unwrap();
            let (y, z) = rest.split_once(", ").unwrap();
            let (x_d, rest) = right.split_once(", ").unwrap();
            let (y_d, z_d) = rest.split_once(", ").unwrap();
            ParametricLine {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
                x_d: x_d.parse().unwrap(),
                y_d: y_d.parse().unwrap(),
                z_d: z_d.parse().unwrap(),
            }
        })
        .collect()
}

fn part_1(input: &str, from: f64, to: f64) -> String {
    let lines = parse(input);
    let sum: usize = 0;
    for two_lines in lines.iter().permutations(2) {
        let line_1 = &two_lines[0];
        let line_2 = &two_lines[1];
        line_1.x
    }
    sum.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input, 200000000000000., 400000000000000.);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(part_1(input, 7., 27.), "2");
    }
}
