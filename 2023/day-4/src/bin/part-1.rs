fn get_numbers(s: &str) -> Vec<u32> {
    let s = s.trim();
    s.split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part_1(input: &str) -> String {
    let mut sum = 0;
    for l in input.lines() {
        let (_, l) = l.split_once(':').unwrap();
        let (left, right) = l.split_once('|').unwrap();
        let winning_numbers = get_numbers(left);
        let hand_numbers = get_numbers(right);

        let numbers = winning_numbers.iter().fold(0, |acc, n| {
            if hand_numbers.contains(n) {
                acc + 1
            } else {
                acc
            }
        });
        if numbers != 0 {
            sum += 2u32.pow(numbers - 1);
        }
    }
    sum.to_string()
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
        let input: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part_1(input), "13");
    }
}
