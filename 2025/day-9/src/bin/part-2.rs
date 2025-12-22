use std::iter;

use day_9::{Point, parse};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }
}

fn part_2(input: &str) -> String {
    let points = parse(input);

    let lines: Vec<Line> = points
        .iter()
        .chain(iter::once(&points[0]))
        .tuple_windows()
        .map(|(p1, p2)| Line::new(p1.clone(), p2.clone()))
        .collect();

    let mut areas: Vec<(isize, Point, Point)> = Vec::with_capacity(points.len().pow(2));

    for (index, p1) in points.iter().enumerate() {
        for p2 in points[index..].iter() {
            let w = (p1.x - p2.x).abs() + 1;
            let h = (p1.y - p2.y).abs() + 1;
            let area = w * h;
            areas.push((area, p1.clone(), p2.clone()));
        }
    }

    areas.sort_by(|a, b| b.cmp(a));

    areas
        .iter()
        .find(|(_, p1, p2)| {
            lines.iter().all(|l| {
                p1.x.max(p2.x) <= l.p1.x.min(l.p2.x)
                    || p1.x.min(p2.x) >= l.p1.x.max(l.p2.x)
                    || p1.y.max(p2.y) <= l.p1.y.min(l.p2.y)
                    || p1.y.min(p2.y) >= l.p1.y.max(l.p2.y)
            })
        })
        .unwrap()
        .0
        .to_string()
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
        let input: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part_2(input), "24");
    }
}
