use std::collections::HashMap;

fn parse(input: &str) -> (Vec<u32>, HashMap<u32, u32>) {
    input.lines().fold((vec![], HashMap::new()), |mut s, l| {
        let mut word_iter = l.split_whitespace();
        let left = word_iter.next().unwrap().parse::<u32>().unwrap();
        let right = word_iter.next().unwrap().parse::<u32>().unwrap();
        s.0.push(left);
        s.1.entry(right).and_modify(|v| *v += 1).or_insert(1);
        s
    })
}

fn part_2(input: &str) -> String {
    let (left, map) = parse(input);
    let result: u32 = left.iter().map(|l| map.get(l).unwrap_or(&0) * l).sum();
    result.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);
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
        assert_eq!(part_2(input), "31");
    }
}
