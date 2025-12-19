use day_3::{get_joltage, parse};

fn part_1(input: &str) -> String {
    let banks = parse(input);
    let mut count = 0;
    for bank in banks {
        count += get_joltage(&bank, 2);
    }

    count.to_string()
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
        let input: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part_1(input), "357");
    }
}
