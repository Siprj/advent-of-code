use std::iter::repeat;

#[derive(Debug)]
struct Card {
    winning_numbers: u32,
}

fn get_numbers(s: &str) -> Vec<u32> {
    let s = s.trim();
    s.split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part_2(input: &str) -> String {
    let mut cards: Vec<Card> = vec![];
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
        cards.push(Card {
            winning_numbers: numbers,
        });
    }

    let mut copies: Vec<u32> = repeat(1).take(cards.len()).collect();

    for (i, c) in cards.iter().enumerate() {
        for ii in 1..=c.winning_numbers {
            copies[ii as usize + i] += copies[i]
        }
    }
    copies.iter().sum::<u32>().to_string()
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
        let input: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part_2(input), "30");
    }
}
