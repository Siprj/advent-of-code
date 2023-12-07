use std::collections::BTreeMap;

fn parse_input(input: &str) -> Vec<(Vec<u8>, u64)> {
    input
        .lines()
        .map(|l| {
            let mut l = l.chars();
            let mut hand: Vec<u8> = Vec::with_capacity(5);
            for _ in 0..5 {
                let c = l.next().expect("Not enought cards in the hand!");
                let card = if let Some(d) = c.to_digit(10) {
                    d as u8
                } else {
                    match c {
                        'T' => 10,
                        'J' => 1, // Special card
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("Card character not recognized!"),
                    }
                };
                hand.push(card);
            }
            l.next()
                .expect("There should be a space between hand and bid!");
            let bid = l
                .collect::<String>()
                .parse()
                .expect("The bid should be a number!");

            (hand, bid)
        })
        .collect()
}

fn part_2(input: &str) -> String {
    let games = parse_input(input);
    let mut clasified_games: Vec<(u8, &Vec<u8>, u64)> = games
        .iter()
        .map(|(hand, bid)| {
            let mut card_distrubution: BTreeMap<u8, u8> = BTreeMap::new();
            for c in hand {
                *card_distrubution.entry(*c).or_insert(0) += 1;
            }
            // Card `1` is joker.
            let jokers = card_distrubution.remove(&1).unwrap_or(0);
            let mut card_distrubution: Vec<(u8, u8)> =
                card_distrubution.iter().map(|(c, r)| (*c, *r)).collect();
            card_distrubution.sort_unstable_by(|c1, c2| c2.1.cmp(&c1.1));

            let rank = if jokers == 5 || card_distrubution[0].1 + jokers >= 5 {
                7
            } else if card_distrubution[0].1 + jokers >= 4 {
                6
            } else if card_distrubution[0].1 + jokers >= 3 {
                let remaining_jokers = card_distrubution[0].1 + jokers;
                if card_distrubution[1].1 + remaining_jokers >= 2 {
                    5
                } else {
                    4
                }
            } else if card_distrubution[0].1 + jokers >= 2 {
                let remaining_jokers = card_distrubution[0].1 + jokers;
                if card_distrubution[1].1 + remaining_jokers >= 2 {
                    3
                } else {
                    2
                }
            } else {
                1
            };

            (rank, hand, *bid)
        })
        .collect();
    clasified_games
        .sort_unstable_by(|(r1, hand1, _), (r2, hand2, _)| r2.cmp(r1).then(hand2.cmp(hand1)));
    clasified_games
        .iter()
        .rev()
        .zip(1..)
        .map(|((_, _, bid), strength)| bid * strength)
        .sum::<u64>()
        .to_string()
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
        let input: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part_2(input), "5905");
    }
}
