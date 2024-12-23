use std::{cmp, collections::HashMap};

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

#[derive(Clone)]
struct Node {
    cost: usize,
    position: (i32, i32),
    path: Vec<char>,
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

fn shortests(start: (i32, i32), end: &(i32, i32), avoid: &(i32, i32)) -> Vec<Vec<char>> {
    if &start == end {
        return vec![vec!['A']];
    }

    let mut stack: Vec<Node> = Vec::with_capacity(100); // This size is quite random :D
    stack.push(Node {
        cost: 0,
        position: start,
        path: Vec::new(),
    });
    let mut ret = Vec::new();

    while let Some(node) = stack.pop() {
        if &node.position == end {
            ret.push(node.path.clone());
            continue;
        }

        if &node.position == avoid {
            continue;
        }

        let d_x = end.1 - node.position.1;
        let d_y = end.0 - node.position.0;

        match d_x.cmp(&0) {
            std::cmp::Ordering::Less => {
                let mut node = node.clone();
                node.path.push('<');
                node.position.1 -= 1;
                stack.push(node);
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                let mut node = node.clone();
                node.path.push('>');
                node.position.1 += 1;
                stack.push(node);
            }
        }

        match d_y.cmp(&0) {
            std::cmp::Ordering::Less => {
                let mut node = node.clone();
                node.path.push('^');
                node.position.0 -= 1;
                stack.push(node);
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                let mut node = node.clone();
                node.path.push('v');
                node.position.0 += 1;
                stack.push(node);
            }
        }
    }

    for p in ret.iter_mut() {
        p.push('A');
    }

    ret
}

fn run_num_pad(input: &Vec<char>) -> usize {
    let num_pad: HashMap<char, (i32, i32)> = HashMap::from_iter(NUM_PAD.iter().copied());
    let mut pos = (3, 2);
    let mut sum = 0;
    for c in input {
        let next = num_pad.get(c).unwrap();
        let shortests = shortests(pos, next, &(3, 0));
        pos = *next;

        sum += shortests.iter().map(run_dir_pad).min().unwrap();
    }
    sum
}

fn run_dir_pad(input: &Vec<char>) -> usize {
    let dir_pad: HashMap<char, (i32, i32)> = HashMap::from_iter(DIR_PAD.iter().copied());
    run_pad(input, &dir_pad, 2, &mut HashMap::new())
}

fn run_pad(
    input: &Vec<char>,
    pad: &HashMap<char, (i32, i32)>,
    depth: usize,
    cache: &mut HashMap<(usize, Vec<char>), usize>,
) -> usize {
    if depth == 0 {
        return input.len();
    }
    if let Some(shortest) = cache.get(&(depth, input.clone())) {
        return *shortest;
    }

    let mut paths: Vec<Vec<Vec<char>>> = vec![vec![]];

    let mut pos = (0, 2);
    for c in input {
        let next = pad.get(c).unwrap();
        let shortests = shortests(pos, next, &(3, 0));
        pos = *next;

        let mut tmp_paths = Vec::new();
        for shortest in shortests.iter() {
            for p in paths.iter() {
                let mut new_path = p.clone();
                new_path.push(shortest.clone());
                tmp_paths.push(new_path);
            }
        }
        paths = tmp_paths;
    }

    let mut min = usize::MAX;
    for path in paths.iter() {
        let mut sum = 0;
        for sub_path in path.iter() {
            sum += run_pad(sub_path, pad, depth - 1, cache);
        }

        min = cmp::min(sum, min);
    }

    cache.insert((depth, input.clone()), min);

    min
}

fn part_1(input: &str) -> String {
    let codes = parse(input);
    let mut sum = 0;
    for code in codes.iter() {
        let len = run_num_pad(code);
        let num: usize = String::from_iter(code[0..3].iter())
            .parse::<usize>()
            .unwrap();
        sum += len * num;
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
        let input: &str = "029A";
        assert_eq!(part_1(input), "1972");
    }

    #[test]
    fn it_works2() {
        let input: &str = "029A
980A
179A
456A
379A";
        assert_eq!(part_1(input), "126384");
    }
}
