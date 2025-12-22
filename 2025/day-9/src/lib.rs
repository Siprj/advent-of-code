#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn add(&self, p2: &(isize, isize)) -> Point {
        Point {
            x: self.x + p2.0,
            y: self.y + p2.1,
        }
    }
}

pub fn parse(input: &str) -> Vec<Point> {
    let input = input.trim();
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}
