use num::Integer;

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect()
}

fn step(stone: u64, next_gen: &mut Vec<u64>) {
    if stone == 0 {
        next_gen.push(1);
    } else if stone.to_string().len().is_even() {
        let n = stone.to_string();
        let len = n.len() / 2;
        next_gen.push(n[0..len].parse::<u64>().unwrap());
        next_gen.push(n[len..].parse::<u64>().unwrap());
    } else {
        next_gen.push(stone * 2024);
    }
}

fn part_1(input: &str) -> String {
    let mut stones = parse(input);
    let mut next_gen = Vec::with_capacity(stones.len() * 2);
    for _ in 0..25 {
        for stone in stones.iter() {
            step(*stone, &mut next_gen);
        }
        stones = next_gen.clone();
        next_gen.clear();
        next_gen.reserve(stones.len() * 2);
    }
    stones.len().to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let len = input.len();
    let result = part_1(&input[..len - 1]);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "125 17";
        assert_eq!(part_1(input), "55312");
    }
}
