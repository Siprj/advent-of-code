fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            let mut word_iter = l.split_whitespace();
            (
                word_iter.next().unwrap().parse::<u32>().unwrap(),
                word_iter.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip()
}

fn part_1(input: &str) -> String {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();

    let result: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    result.to_string()
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
        let input: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part_1(input), "11");
    }
}
