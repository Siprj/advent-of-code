use std::collections::HashMap;

use num::Integer;

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect()
}

fn step(stone: u64, num: u64, next_gen: &mut HashMap<u64, u64>) {
    if stone == 0 {
        *next_gen.entry(1).or_default() += num;
    } else if stone.to_string().len().is_even() {
        let n = stone.to_string();
        let len = n.len() / 2;
        *next_gen
            .entry(n[0..len].parse::<u64>().unwrap())
            .or_default() += num;
        *next_gen
            .entry(n[len..].parse::<u64>().unwrap())
            .or_default() += num;
    } else {
        next_gen
            .entry(stone * 2024)
            .and_modify(|v| *v += num)
            .or_insert(num);
    }
}

fn part_2(iterations: usize, input: &str) -> String {
    let mut stones: HashMap<u64, u64> = parse(input).iter().copied().map(|v| (v, 1)).collect();
    let mut next_gen = HashMap::with_capacity(stones.len() * 2);
    for _ in 0..iterations {
        for (stone, num) in stones.iter() {
            step(*stone, *num, &mut next_gen);
        }
        stones = next_gen.clone();
        next_gen.clear();
        next_gen.reserve(stones.len() * 2);
    }
    stones.values().sum::<u64>().to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let len = input.len();
    let result = part_2(75, &input[..len - 1]);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "125 17";
        assert_eq!(part_2(25, input), "55312");
    }
}
