use day_2::parse;
use num::integer::div_rem;

fn part_2(input: &str) -> String {
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
    let len = str.len();
    'outer: for i in 0..(len / 2) {
        let pattern = &str[0..=i];
        let pattern_len = pattern.len();
        let (div, rem) = div_rem(len, pattern_len);
        if rem != 0 {
            continue;
        }

        for y in 1..div {
            if pattern != &str[y * pattern_len..(y + 1) * pattern_len] {
                continue 'outer;
            }
        }
        return true;
    }
    false
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
        let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part_2(input), "4174379265");
    }
}
