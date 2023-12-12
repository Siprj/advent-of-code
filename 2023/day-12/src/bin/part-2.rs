#![feature(iter_intersperse)]
use std::{iter::repeat, collections::HashMap};
use rayon::prelude::*;

fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input.lines().map(|l|{
        let (gears, counts) = l.split_once(' ').unwrap();
        let counts = repeat(counts.split(',').map(|s| s.parse::<usize>().unwrap())).take(5).flatten().collect();

        (repeat(gears).take(5).intersperse("?").flat_map(|v| v.chars()).collect(),counts)
    }).collect()
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
    current_gear: usize,
    current_check: usize,
    diff: usize,
}

fn sol(gears: &[char], checks: &[usize], store: &mut HashMap<Position, usize>, pos: &Position) -> usize {
    if let Some(count) = store.get(pos) {
        return *count;
    }

    if checks[pos.current_check..].is_empty(){
        if gears[pos.current_gear..].iter().all(|c| c == &'.' || c == &'?') {
            return 1;
        } else {
            return 0;
        }
    }

    let Some(current) = gears[pos.current_gear..].first() else {
        if pos.diff > 0 && checks.len() - 1== pos.current_check && checks[pos.current_check] == pos.diff {
            return 1;
        } else {
            return 0;
        }
    };

    match current {
        '#' => {
            sol(gears, checks, store, &Position{
                current_gear: pos.current_gear + 1,
                current_check: pos.current_check,
                diff: pos.diff+1,
            })
        },
        '.' => {
            if pos.diff > 0 {
                if pos.diff == checks[pos.current_check] {
                    sol(gears, checks, store, &Position{
                        current_gear: pos.current_gear + 1,
                        current_check: pos.current_check + 1,
                        diff: 0,
                    })
                } else {
                    0
                }
            } else {
                sol(gears, checks, store, &Position{
                        current_gear: pos.current_gear + 1,
                        current_check: pos.current_check,
                        diff: 0,
                    })
            }
        },
        '?' => {
            let c1 = if pos.diff > 0 {
                if pos.diff == checks[pos.current_check] {
                    let pos = Position{
                        current_gear: pos.current_gear + 1,
                        current_check: pos.current_check + 1,
                        diff: 0,
                    };
                    let tmp = sol(gears, checks, store, &pos);
                    store.insert(pos, tmp);
                    tmp
                } else {
                    0
                }
            } else {
                let pos = Position{
                        current_gear: pos.current_gear + 1,
                        current_check: pos.current_check,
                        diff: 0,
                    };
                let tmp = sol(gears, checks, store, &pos);
                store.insert(pos, tmp);
                tmp
            };

            let pos = Position{
                        current_gear: pos.current_gear + 1,
                        current_check: pos.current_check,
                        diff: pos.diff + 1,
                    };
            let c2 = sol(gears, checks, store, &pos);
            store.insert(pos, c2);
            c2 + c1
        },
        _ => 0,
    }
}

fn solution_count(input: (Vec<char>, Vec<usize>)) -> usize {
    let (gears, checks) = input;
    let mut store = HashMap::new();
    sol(&gears, &checks, &mut store, &Position{ current_gear: 0, current_check: 0, diff: 0 })
}

fn part_2(input: &str) -> String {
    let inputs = parse(input);

    let count: usize = inputs.into_par_iter().map(solution_count).sum();
    count.to_string()
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
        let input: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part_2(input), "525152");
    }

    #[test]
    fn it_works_2() {

        let input = ".# 1";
        assert_eq!(parse(input), [(vec!['.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#'], vec![1, 1, 1, 1, 1])]);
    }

    #[test]
    fn it_works_3() {
        let input: &str = "???.### 1,1,3";
        assert_eq!(part_2(input), "1");
    }

}
