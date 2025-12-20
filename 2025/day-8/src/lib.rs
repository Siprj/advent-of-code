#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point {
    pub fn distance_squared(&self, p2: &Point) -> isize {
        (self.x - p2.x).pow(2) + (self.y - p2.y).pow(2) + (self.z - p2.z).pow(2)
    }
}

pub fn parse(input: &str) -> Vec<Point> {
    let input = input.trim();
    input
        .lines()
        .map(|l| {
            let (x, l) = l.split_once(',').unwrap();
            let (y, z) = l.split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
            }
        })
        .collect()
}
