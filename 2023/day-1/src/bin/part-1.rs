fn part_1(input: &str) -> String {
    let result: u32 = input
        .split_whitespace()
        .map(|l| {
            let first = l
                .chars()
                .find(|c| c.is_ascii_digit())
                .and_then(|c| c.to_digit(10))
                .unwrap();
            let last = l
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .and_then(|c| c.to_digit(10))
                .unwrap();
            (first * 10) + last
        })
        .sum();
    result.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part_1(input), "142");
    }
}
