use itertools::Itertools;
use ndarray::array;
use ndarray_linalg::Solve;

struct ParametricLine {
    x: f64,
    y: f64,
    x_d: f64,
    y_d: f64,
}

fn parse(input: &str) -> Vec<ParametricLine> {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(" @ ").unwrap();
            let (x, rest) = left.split_once(", ").unwrap();
            let (y, _z) = rest.split_once(", ").unwrap();
            let (x_d, rest) = right.split_once(", ").unwrap();
            let (y_d, _z_d) = rest.split_once(", ").unwrap();
            ParametricLine {
                x: x.trim().parse().unwrap(),
                y: y.trim().parse().unwrap(),
                x_d: x_d.trim().parse().unwrap(),
                y_d: y_d.trim().parse().unwrap(),
            }
        })
        .collect()
}

fn part_1(input: &str, from: f64, to: f64) -> String {
    let lines = parse(input);
    let mut sum: usize = 0;
    for (line_1, line_2) in lines.iter().tuple_combinations() {
        let a = array![[line_1.x_d, -line_2.x_d], [line_1.y_d, -line_2.y_d]];
        let b = array![line_2.x - line_1.x, line_2.y - line_1.y];
        match a.solve(&b) {
            Ok(ts) => {
                let intersection_x = (line_1.x_d * ts[0]) + line_1.x;
                let intersection_y = (line_1.y_d * ts[0]) + line_1.y;
                // let z_1 = line_1.z_d * ts[0] + line_1.z;
                // let z_2 = line_2.z_d * ts[1] + line_2.z;
                //println!("z_1: {}, z_2: {}", z_1, z_2);
                if ts[0] >= 0.
                    && ts[1] >= 0.
                    && intersection_x >= from
                    && intersection_x <= to
                    && intersection_y >= from
                    && intersection_y <= to
                /*&& ((z_1 - z_2).abs() <= std::f64::EPSILON)*/
                {
                    sum += 1;
                }
            }
            Err(_) => {}
        }
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
