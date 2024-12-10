use std::{cmp::min, collections::VecDeque, iter};

use itertools::Itertools;

fn parse(input: &str) -> VecDeque<(usize, (u8, u8))> {
    input.as_bytes().iter().map(|c| (c - 48) ).chain(iter::once(0)).tuples().enumerate().collect()
}

fn part_1(input: &str) -> String {
    let mut segments = parse(input);
    let mut fragmented: Vec<usize> = Vec::with_capacity(segments.len() * 10);

    while let Some((index, (used, free))) = segments.pop_front() {
        for _ in 0..used {
            fragmented.push(index);
        }
        if free > 0 {
            if let Some((back_index, (back_used, _))) = segments.pop_back(){
                for _ in 0..min(free, back_used) {
                    fragmented.push(back_index);
                }
                match free.cmp(&back_used) {
                    std::cmp::Ordering::Less => {
                        segments.push_back((back_index, (back_used - free, 0)));
                    },
                    std::cmp::Ordering::Equal => {},
                    std::cmp::Ordering::Greater => {
                        segments.push_front((index, (0, free - back_used)));
                    },
                }
            }
        }
    }

    let mut check_sum = 0;
    for (i, v) in fragmented.iter().enumerate() {
        check_sum += i * v;
    }

    check_sum.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let len = input.len();
    let result = part_1(&input[..len-1]);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "2333133121414131402";
        assert_eq!(part_1(input), "1928");
    }
}
