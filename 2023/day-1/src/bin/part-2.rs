use std::fs::read_to_string;

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

fn main() {
    let content: String = read_to_string("input.txt").unwrap();
    let res: u32 = content
        .split_whitespace()
        .map(|l| {
            let first = get_first_number(l);
            let last = get_last_number(l);
            (first * 10) + last
        })
        .sum();
    println!("sum: {}", res);
}
