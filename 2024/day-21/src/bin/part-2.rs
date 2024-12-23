use std::{collections::HashMap, iter::once};

use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

const NUM_PAD: [(char, (i32, i32)); 11] = [
    ('7', (0, 0)),
    ('8', (0, 1)),
    ('9', (0, 2)),
    ('4', (1, 0)),
    ('5', (1, 1)),
    ('6', (1, 2)),
    ('1', (2, 0)),
    ('2', (2, 1)),
    ('3', (2, 2)),
    ('0', (3, 1)),
    ('A', (3, 2)),
];

const DIR_PAD: [(char, (i32, i32)); 5] = [
    ('^', (0, 1)),
    ('A', (0, 2)),
    ('<', (1, 0)),
    ('v', (1, 1)),
    ('>', (1, 2)),
];

fn shortests(start: &(i32, i32), end: &(i32, i32)) -> Vec<char> {
    if start == end {
        return vec![];
    }

    let d_x = end.1 - start.1;
    let d_y = end.0 - start.0;
    let mut ret = Vec::new();

    for _ in 0..d_x.abs() {
        match d_x.cmp(&0) {
            std::cmp::Ordering::Less => {
                ret.push('<');
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                ret.push('>');
            }
        }
    }

    for _ in 0..d_y.abs() {
        match d_y.cmp(&0) {
            std::cmp::Ordering::Less => {
                ret.push('^');
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                ret.push('v');
            }
        }
    }
    ret
}

type Cache = HashMap<(char, char, usize), usize>;

fn validate_path(
    mut pos: (i32, i32),
    path: &[&char],
    avoid: &(i32, i32),
    pad_size: &(i32, i32),
) -> bool {
    path.iter().all(|c| {
        let next_pos: (i32, i32) = match c {
            '^' => (pos.0 - 1, pos.1),
            'v' => (pos.0 + 1, pos.1),
            '<' => (pos.0, pos.1 - 1),
            '>' => (pos.0, pos.1 + 1),
            _ => {
                panic!()
            }
        };
        if &next_pos == avoid
            || pos.0 < 0
            || pos.0 >= pad_size.0
            || pos.1 < 0
            || pos.1 >= pad_size.1
        {
            return false;
        }
        pos = next_pos;
        true
    })
}

fn run(
    depth: usize,
    start: char,
    end: char,
    cache: &mut Cache,
    pad: &HashMap<char, (i32, i32)>,
    dir_pad: &HashMap<char, (i32, i32)>,
    avoid: &(i32, i32),
    pad_size: &(i32, i32),
) -> usize {
    let start_pos = pad.get(&start).unwrap();
    let end_pos = pad.get(&end).unwrap();

    if depth == 0 {
        let ret = (start_pos.0 - end_pos.0).unsigned_abs() as usize
            + (start_pos.1 - end_pos.1).unsigned_abs() as usize
            + 1;
        return ret;
    }

    if let Some(v) = cache.get(&(start, end, depth)) {
        return *v;
    }

    let path = shortests(start_pos, end_pos);

    let ret: usize = path
        .iter()
        .permutations(path.len())
        .map(|moves| {
            if validate_path(*start_pos, &moves, avoid, pad_size) {
                once('A')
                    .chain(moves.into_iter().copied())
                    .chain(once('A'))
                    .tuple_windows()
                    .map(|(start, end)| {
                        run(
                            depth - 1,
                            start,
                            end,
                            cache,
                            dir_pad,
                            dir_pad,
                            &(0, 0),
                            &(2, 3),
                        )
                    })
                    .sum()
            } else {
                usize::MAX
            }
        })
        .min()
        .unwrap();

    cache.insert((start, end, depth), ret);
    ret
}

fn run_num_pad(code: &[char], depth: usize) -> usize {
    let dir_pad: HashMap<char, (i32, i32)> = HashMap::from_iter(DIR_PAD.iter().copied());
    let num_pad: HashMap<char, (i32, i32)> = HashMap::from_iter(NUM_PAD.iter().copied());

    once('A')
        .chain(code.iter().copied())
        .tuple_windows()
        .map(|(start, end)| {
            let mut cache: Cache = HashMap::new();
            run(
                depth,
                start,
                end,
                &mut cache,
                &num_pad,
                &dir_pad,
                &(3, 0),
                &(4, 3),
            )
        })
        .sum()
}

fn part_2(input: &str, depth: usize) -> String {
    let codes = parse(input);
    let mut sum = 0;
    for code in codes.iter() {
        let len = run_num_pad(code, depth);
        let num: usize = String::from_iter(code[0..3].iter())
            .parse::<usize>()
            .unwrap();
        sum += len * num;
    }
    sum.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input, 25);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "029A";
        assert_eq!(part_2(input, 2), "1972");
    }

    #[test]
    fn it_works2() {
        let input: &str = "980A";
        assert_eq!(part_2(input, 2), "58800");
    }

    #[test]
    fn it_works3() {
        let input: &str = "179A";
        assert_eq!(part_2(input, 2), "12172");
    }

    #[test]
    fn it_works4() {
        let input: &str = "029A
980A
179A
456A
379A";
        assert_eq!(part_2(input, 2), "126384");
    }
}
