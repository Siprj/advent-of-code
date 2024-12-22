use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> Vec<u64> {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect()
}

fn mix(secret: &mut u64, new_value: u64) {
    *secret ^= new_value;
}

fn prune(secret: &mut u64) {
    *secret %= 16777216;
}

fn random(mut secret: u64) -> u64 {
    let mul = secret * 64;
    mix(&mut secret, mul);
    prune(&mut secret);
    let div = secret / 32;
    mix(&mut secret, div);
    prune(&mut secret);
    let mul = secret * 2048;
    mix(&mut secret, mul);
    prune(&mut secret);
    secret
}

fn part_2(input: &str) -> String {
    let numbers = parse(input);
    let mut all_changes: HashMap<(i8, i8, i8, i8), u64> = HashMap::new();
    for n in numbers.iter() {
        let mut secret = *n;
        let mut old_digit = (secret % 10) as i8;
        let mut diff1 = 0;
        let mut diff2 = 0;
        let mut diff3 = 0;
        let mut visited: HashSet<(i8, i8, i8, i8)> = HashSet::new();
        for i in 0..2000u64 {
            secret = random(secret);
            let digit = (secret % 10) as i8;
            let diff = digit - old_digit;
            old_digit = digit;
            if i >= 3 && !visited.contains(&(diff1, diff2, diff3, diff)) {
                *all_changes.entry((diff1, diff2, diff3, diff)).or_insert(0) += digit as u64;
                visited.insert((diff1, diff2, diff3, diff));
            }
            diff1 = diff2;
            diff2 = diff3;
            diff3 = diff;
        }
    }

    all_changes.values().max().unwrap().to_string()
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
        let input: &str = "1
2
3
2024";
        assert_eq!(part_2(input), "23");
    }
}
