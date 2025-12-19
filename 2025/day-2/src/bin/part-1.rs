use day_2::parse;
use num::Integer;

fn part_1(input: &str) -> String {
    let ranges = parse(input);
    let mut count = 0;

    for range in ranges {
        println!("processing range: {:?}", range);
        count += process_range(range);
    }

    count.to_string()
}

fn process_range((from, to): (u64, u64)) -> u64 {
    let mut acc = 0;
    for num in from..=to {
        if check_number(num) {
            acc += num;
        }
    }
    acc
}

fn check_number(num: u64) -> bool {
    let str = num.to_string();
    if str.len().is_even() {
        str[..(str.len() / 2)] == str[(str.len() / 2)..]
    } else {
        false
    }
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
        let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part_1(input), "1227775554");
    }
}
