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

fn part_1(input: &str) -> String {
    let numbers = parse(input);
    let mut sum = 0;
    for n in numbers.iter() {
        let mut secret = *n;
        for _ in 0..2000u64 {
            secret = random(secret);
        }
        sum += secret;
    }

    sum.to_string()
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
        let input: &str = "1
10
100
2024";
        assert_eq!(part_1(input), "37327623");
    }
}
