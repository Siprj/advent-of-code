#[derive(Debug, Clone)]
pub struct Interval {
    pub start: usize,
    pub end: usize,
}

impl Interval {
    pub fn new(start: usize, end: usize) -> Interval {
        Self { start, end }
    }
    pub fn intersect(&self, value: usize) -> bool {
        self.start <= value && self.end >= value
    }
    pub fn overlap(&self, interval: &Interval) -> bool {
        self.intersect(interval.start)
            || self.intersect(interval.end)
    }
}

pub fn parse(input: &str) -> (Vec<Interval>, Vec<usize>) {
    let input = input.trim();
    let (intervals, ids) = input.split_once("\n\n").unwrap();
    let intervals = intervals
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();
            Interval::new(start, end)
        })
        .collect();
    let ids = ids.lines().map(|line| line.parse().unwrap()).collect();
    (intervals, ids)
}
