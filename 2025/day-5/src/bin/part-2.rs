use std::cmp::{max, min};

use day_5::{parse, Interval};

fn part_2(input: &str) -> String {
    let (mut intervals, _ids) = parse(input);
    let mut count: usize = 0;
    intervals.sort_by(|v1, v2| v1.start.cmp(&v2.start));

    let intervals = unify(&intervals);

    for interval in &intervals {
        count += (interval.end - interval.start) + 1;
    }

    count.to_string()
}

fn unify(intervals: &[Interval]) -> Vec<Interval> {
    let mut ret: Vec<Interval> = vec![];

    let mut acc = intervals[0].clone();

    for interval in &intervals[1..] {
        if acc.overlap(interval) {
            acc.end = max(interval.end, acc.end);
            acc.start = min(interval.start, acc.start);
        } else {
            ret.push(acc.clone());
            acc = interval.clone();
        }
    }
    ret.push(acc);
    ret
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
        let input: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part_2(input), "14");
    }
}
