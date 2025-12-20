use day_5::{parse, Interval};

fn part_1(input: &str) -> String {
    let (intervals, ids) = parse(input);
    let mut count: usize = 0;
    for id in ids {
        if is_fresh(&intervals, id) {
            count += 1;
        }
    }
    count.to_string()
}

fn is_fresh(intervals: &[Interval], id: usize) -> bool {
    for interval in intervals {
        if interval.intersect(id) {
            return true;
        }
    }
    false
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);
    println!("Part 1: {}", result);
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
        assert_eq!(part_1(input), "3");
    }
}
