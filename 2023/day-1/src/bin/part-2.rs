const NUMBERS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_first_number(str: &str) -> u32 {
    for (i, c) in str.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            return d;
        }
        for (w, n) in NUMBERS {
            if str[i..].starts_with(w) {
                return n;
            }
        }
    }
    unreachable!()
}

fn get_last_number(str: &str) -> u32 {
    for (i, c) in str.chars().rev().enumerate() {
        if let Some(d) = c.to_digit(10) {
            return d;
        }
        for (w, n) in NUMBERS {
            if str[..str.len() - i].ends_with(w) {
                return n;
            }
        }
    }
    unreachable!()
}

fn part_2(input: &str) -> String {
    let result: u32 = input
        .split_whitespace()
        .map(|l| {
            let first = get_first_number(l);
            let last = get_last_number(l);
            (first * 10) + last
        })
        .sum();
    result.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part_2(input), "281");
    }
}
