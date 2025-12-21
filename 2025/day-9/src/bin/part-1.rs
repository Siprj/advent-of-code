use std::cmp;

use day_9::parse;

fn part_1(input: &str) -> String {
    let points = parse(input);

    let mut max = 0;

    for p1 in points.iter() {
        for p2 in points.iter() {
            let w = p1.0 - p2.0 + 1;
            let h = p1.1 - p2.1 + 1;
            let area = (w * h).abs();
            max = cmp::max(area, max);
        }
    }

    max.to_string()
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
        let input: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part_1(input), "50");
    }
}
