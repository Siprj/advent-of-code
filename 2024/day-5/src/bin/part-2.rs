use std::{cmp::Ordering, collections::HashSet};

fn parse(input: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rule_set: HashSet<(u32, u32)> = rules.lines().map(|l|{
        let (p1, p2) = l.split_once('|').unwrap();
        let p1 = p1.parse::<u32>().unwrap();
        let p2 = p2.parse::<u32>().unwrap();
        (p1, p2)
    }).collect();
    let updates = updates.lines().map(|l| l.split(',').map(|p| p.parse().unwrap()).collect()).collect();
    (rule_set, updates)
}

fn part_2(input: &str) -> String {
    let (rule_set, updates) = parse(input);
    let mut unordered : Vec<Vec<u32>> = vec![];
    'line: for l in updates.into_iter() {
        for (i, p1) in l.iter().enumerate() {
            for p2 in l[0..i].iter() {
                if rule_set.contains(&(*p1, *p2)) {
                    unordered.push(l);
                    continue 'line;
                }
            }
        }
    }

    for l in unordered.iter_mut() {
        l.sort_by(|a, b|{
            if rule_set.contains(&(*a, *b)) {
                Ordering::Less
            } else if rule_set.contains(&(*b, *a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
    }
    let mut count = 0;
    for l in unordered {
        let middle = l.len() / 2;
        count += l[middle];
    }

    count.to_string()
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
        let input: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part_2(input), "123");
    }
}
