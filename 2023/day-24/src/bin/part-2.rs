use std::str::LinesAny;

use itertools::Itertools;
use ndarray::{array, Array, Array1, Array2};
use ndarray_linalg::{Inverse, Scalar, Solve};

#[derive(Debug, Clone)]
struct ParametricLine {
    p: Array1<f64>,
    v: Array1<f64>,
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
            let x = x.trim().parse().unwrap();
            let y = y.trim().parse().unwrap();
            let z = z.trim().parse().unwrap();
            let x_d = x_d.trim().parse().unwrap();
            let y_d = y_d.trim().parse().unwrap();
            let z_d = z_d.trim().parse().unwrap();
            ParametricLine {
                p: array![x, y, z],
                v: array![x_d, y_d, z_d],
            }
        })
        .collect()
}

fn cross(a: &Array1<f64>, b: &Array1<f64>) -> Array1<f64> {
    array![
       a[1] * b[2] - a[2] * b[1],
       a[2] * b[0] - a[0] * b[2],
       a[0] * b[1] - a[1] * b[0],
    ]
}

fn abs(a: &Array1<f64>) -> f64{
    a.iter().map(|v| v * v).sum::<f64>().sqrt()
}

fn find_intersection_3d(line_1: &ParametricLine, line_2: &ParametricLine) -> bool {
    // cred https://math.stackexchange.com/questions/270767/find-intersection-of-two-3d-lines
    let g = &line_2.p - &line_1.p;
    let h = abs(&cross(&line_2.v, &g));
    let k = abs(&cross(&line_2.v, &line_1.v));
    h == 0.0 || k == 0.0
}
fn intersection(line_1: &ParametricLine, line_2: &ParametricLine) -> bool {
    let a = array![[line_1.v[0], -line_2.v[0]], [line_1.v[1], -line_2.v[1]]];
    let b = array![line_2.p[0] - line_1.p[0], line_2.p[1] - line_1.p[1]];
    match a.solve(&b) {
        Ok(ts) => {
            let z_1 = line_1.v[2] * ts[0] + line_1.p[2];
            let z_2 = line_2.v[2] * ts[1] + line_2.p[2];
            let diff = (z_1 - z_2).abs();
            //println!("this is .... diff: {}, z_1: {}, z_2: {}", diff, z_1, z_2);
            if diff <= 1000.0 {
                println!("found intersection: diff: {}, line_1: {:?}", diff, &line_1);
                return true;
            }
        }
        Err(_) => {}
    }
    false
}

fn make_equation(line_1: &ParametricLine, line_2: &ParametricLine) -> (Vec<f64>, f64) {
    (
        vec![
            line_2.v[1] - line_1.v[1],
            line_1.v[0] - line_2.v[0],
            line_1.p[1] - line_2.p[1],
            line_2.p[0] - line_1.p[0],
        ],
        line_1.v[0] * line_1.p[1] - line_2.v[0] * line_2.p[1] + line_2.p[0] * line_2.v[1]
            - line_1.p[0] * line_1.v[1],
    )
}

fn part_2(input: &str) -> String {
    let lines = parse(input);
    let (a1, b1) = make_equation(&lines[0], &lines[1]);
    let (a2, b2) = make_equation(&lines[0], &lines[2]);
    let (a3, b3) = make_equation(&lines[0], &lines[3]);
    let (a4, b4) = make_equation(&lines[0], &lines[4]);
    let matrix: Array2<f64> = array![
        [a1[0], a1[1], a1[2], a1[3]],
        [a2[0], a2[1], a2[2], a2[3]],
        [a3[0], a3[1], a3[2], a3[3]],
        [a4[0], a4[1], a4[2], a4[3]]
    ];

    let result = matrix.inv().unwrap().dot(&array![b1, b2, b3, b4]);
    let a = result[0]; // corresponds to stone x
    let b = result[1]; // corresponds to stone y
    let d = result[2]; // corresponds to stone d_x
    let e = result[3]; // corresponds to stone d_y

    let t1 = (a - lines[0].p[0]) / (lines[0].v[0] - d);
    let t2 = (a - lines[1].p[0]) / (lines[1].v[0] - d);
    let f = (((lines[0].p[2] - lines[1].p[2]) + t1 * lines[0].v[2] - t2 * lines[1].v[2]) / (t1 - t2)).round();
    let c = (lines[0].p[2] + t1 * (lines[0].v[2] - f)).round();

    let spred: i64 = 60;
    let spred = (0..spred).map(|v| v - (spred/2));

    dbg!(a, b, c, d ,e ,f);

    for pos_vel in spred.combinations_with_replacement(6) {
        //println!("diff: {:?}", pos_vel);
        let rock_line = ParametricLine{
            p: array![a + pos_vel[0] as f64, b + pos_vel[1] as f64, c  + pos_vel[2] as f64],
            v: array![d + pos_vel[3] as f64, e + pos_vel[4] as f64, f  + pos_vel[5] as f64],
        };
        //println!("rock_line: {:?}", rock_line);
        if lines.iter().take(5).all(|l|find_intersection_3d(&rock_line, l)) {
            println!("found it");
            return rock_line.p.sum().to_string();
        }
    }

    let sum: usize = (a.abs() + b.abs() + c.abs()).ceil() as usize;
    sum.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);
    println!("Result: {}", result);
}
