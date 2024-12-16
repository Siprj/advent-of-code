use std::collections::HashSet;

#[derive(Debug)]
struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(' ').unwrap();
            let (px, py) = p.split_once(',').unwrap();
            let p = (px[2..].parse::<i64>().unwrap(), py.parse::<i64>().unwrap());
            let (vx, vy) = v.split_once(',').unwrap();
            let v = (vx[2..].parse::<i64>().unwrap(), vy.parse::<i64>().unwrap());
            Robot { p, v }
        })
        .collect()
}

const SECONDS: i64 = 100;

fn print_map(robots: &[Robot], width: i64, height: i64) {
    let mut map: Vec<Vec<i64>> = (0..height)
        .map(|_| (0..width).map(|_| 0).collect())
        .collect();
    for r in robots.iter() {
        map[r.p.1 as usize][r.p.0 as usize] += 1;
    }
    for l in map.iter() {
        for n in l.iter() {
            if *n > 0 {
                print!("{n}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_1(input: &str, width: i64, height: i64) -> String {
    let mut robots = parse(input);

    for i in 0..10000 {
        for robot in robots.iter_mut() {
            robot.p = (
                (robot.p.0 + robot.v.0).rem_euclid(width),
                (robot.p.1 + robot.v.1).rem_euclid(height),
            );
        }
        let set: HashSet<(i64, i64)> = robots.iter().map(|v| v.p).collect();
        if set.len() == robots.len() {
            print_map(&robots, width, height);
            return (i + 1).to_string();
        }
    }
    "".to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input, 101, 103);
    println!("Part 1: {}", result);
}
