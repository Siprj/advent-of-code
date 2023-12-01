use std::fs::read_to_string;

fn main() {
    let content: String = read_to_string("input.txt").unwrap();
    let res: u32 = content
        .split_whitespace()
        .map(|l| {
            let first = l
                .chars()
                .find(|c| c.is_digit(10))
                .and_then(|c| c.to_digit(10))
                .unwrap();
            let last = l
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .and_then(|c| c.to_digit(10))
                .unwrap();
            (first * 10) + last
        })
        .sum();
    println!("sum: {}", res);
}
