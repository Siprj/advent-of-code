use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, iter::{once, repeat, OnceWith}};

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.bytes().collect()
        }).collect()
}

const NUM_PAD: [(u8, (i32, i32)); 11] = [
    (b'7', (0, 0)),
    (b'8', (0, 1)),
    (b'9', (0, 2)),
    (b'4', (1, 0)),
    (b'5', (1, 1)),
    (b'6', (1, 2)),
    (b'1', (2, 0)),
    (b'2', (2, 1)),
    (b'3', (2, 2)),
    (b'0', (3, 1)),
    (b'A', (3, 2)),
];

const DIR_PAD: [(u8, (i32, i32)); 5] = [
    (b'^', (0, 1)),
    (b'A', (0, 2)),
    (b'<', (1, 0)),
    (b'v', (1, 1)),
    (b'>', (1, 2)),
];

struct Node {
    cost: usize,
    depth: usize,
    position: (i32, i32),
    nodes: BinaryHeap<Reverse<Node>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn shortests(start: (i32, i32), end: &(i32, i32), avoid: &(i32, i32)) -> Vec<Vec<u8>> {
    if &start == end {
        return vec![vec![]];
    }

    let mut ret: Vec<Vec<u8>> = Vec::new();
    let d_x = end.1 - start.1;
    let d_y = end.0 - start.0;
    if d_x > 0 {
        let pos = (start.0, start.1 + 1);
        if &start != avoid {
            let nexts = shortests(pos, end, avoid);
            for n in nexts {
                ret.push(Vec::from_iter(once(b'>').chain(n.iter().copied())))
            }
        }
    } else if d_x < 0 {
        let pos = (start.0, start.1 - 1);
        if &start != avoid {
            let nexts = shortests(pos, end, avoid);
            for n in nexts {
                ret.push(Vec::from_iter(once(b'<').chain(n.iter().copied())))
            }
        }
    }

    if d_y > 0 {
        let pos = (start.0 + 1, start.1);
        if &start != avoid {
            let nexts = shortests(pos, end, avoid);
            for n in nexts {
                ret.push(Vec::from_iter(once(b'v').chain(n.iter().copied())))
            }
        }
    } else if d_y < 0 {
        let pos = (start.0 - 1, start.1);
        if &start != avoid {
            let nexts = shortests(pos, end, avoid);
            for n in nexts {
                ret.push(Vec::from_iter(once(b'^').chain(n.iter().copied())))
            }
        }
    }

    //println!("asdf: {ret:?}");
    ret
}


fn run_pad(input: &Vec<u8>, mut pos: (i32, i32), avoid: (i32, i32), pad: &HashMap<u8, (i32, i32)>) -> Vec<Vec<u8>>{
    let mut ret: Vec<Vec<u8>>  = Vec::with_capacity(1000);
    ret.push(Vec::new());
    for c in input {
        let next = pad.get(c).unwrap();
        let next_shortest = shortests(pos, next, &avoid);
        pos = *next;
        let cloned = ret.clone();
        ret.clear();

        for c in cloned.iter() {
            for n in next_shortest.iter() {
                ret.push(Vec::from_iter(c.iter().copied().chain(n.iter().copied()).chain(once(b'A'))));
            }
        }
    }
    ret
}

fn run_num_pad(input: &Vec<u8>) -> Vec<Vec<u8>> {
    let num_pad: HashMap<u8, (i32, i32)>  = HashMap::from_iter(NUM_PAD.iter().copied());
    run_pad(input, (3, 2), (3,0), &num_pad)
}

fn run_dir_pad(input: &Vec<u8>) -> Vec<Vec<u8>> {
    let dir_pad: HashMap<u8, (i32, i32)>  = HashMap::from_iter(DIR_PAD.iter().copied());
    run_pad(input, (0, 2), (0,0), &dir_pad)
}

fn part_1(input: &str) -> String {
    let codes = parse(input);
    let mut sum = 0;
    for code in codes.iter() {
        let mut kwa: Vec<u8> = Vec::new();
        let next = run_num_pad(code);
        for n in next {
            let next = run_dir_pad(&n);
            for n in next.iter() {
                let mut next = run_dir_pad(n);
                next.sort_by_key(|s1| s1.len());
                if kwa.is_empty() || next[0].len() < kwa.len() {
                    kwa = next[0].clone();
                }
            }
        }
        let num: usize = String::from_iter(code[0..3].iter().map(|c| *c as char)).parse::<usize>().unwrap();
        sum += kwa.len() * num;
    }
    sum.to_string()
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
        let input: &str = "029A
980A
179A
456A
379A";
        assert_eq!(part_1(input), "126384");
    }
}
