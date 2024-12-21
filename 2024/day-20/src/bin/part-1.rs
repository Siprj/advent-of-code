use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Clone)]
struct Map<T> {
    map: Vec<Vec<T>>,
    width: i32,
    height: i32,
}

impl<T: Copy + Eq> Map<T> {
    fn from_value(v: T, width: i32, height: i32) -> Map<T> {
        let map = (0..height)
            .map(|_| (0..width).map(|_| v).collect())
            .collect();
        Map { map, width, height }
    }
    fn get_unsafe(&self, pos: &Pos) -> T {
        self.map[pos.y as usize][pos.x as usize]
    }
    fn get(&self, pos: &Pos) -> Option<T> {
        if self.is_within(pos) {
            Some(self.get_unsafe(pos))
        } else {
            None
        }
    }
    fn set(&mut self, pos: &Pos, v: T) {
        if self.is_within(pos) {
            self.map[pos.y as usize][pos.x as usize] = v;
        } else {
            panic!("Pos: {pos:?} is outside of the map");
        }
    }
    fn is_within(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }
}

fn is_valid(map: &Map<u8>, pos: &Pos) -> bool {
    map.is_within(pos) && map.get_unsafe(pos) != b'#'
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn add(&self, (y, x): &(i32, i32)) -> Pos {
        Pos {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

fn parse(input: &str) -> (Pos, Pos, Map<u8>) {
    let map: Vec<Vec<u8>> = input.trim().lines().map(|l| l.bytes().collect()).collect();
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, v)| {
                if *v == b'S' {
                    Some(Pos {
                        x: x as i32,
                        y: y as i32,
                    })
                } else {
                    None
                }
            })
        })
        .unwrap();
    let end = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, v)| {
                if *v == b'E' {
                    Some(Pos {
                        x: x as i32,
                        y: y as i32,
                    })
                } else {
                    None
                }
            })
        })
        .unwrap();
    (start, end, Map { map, width, height })
}

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const DIR2S: [(i32, i32); 4] = [(0, 2), (2, 0), (0, -2), (-2, 0)];

#[derive(Clone)]
struct Node {
    step_count: usize,
    position: Pos,
    steps: Vec<Pos>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.steps.eq(&other.steps)
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
        self.steps.cmp(&other.steps)
    }
}

fn find_path(
    start: &Pos,
    step_count: usize,
    end: &Pos,
    map: &Map<u8>,
    cache: &mut Map<usize>,
) -> Option<Node> {
    let mut min_heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    min_heap.push(Reverse(Node {
        step_count,
        position: *start,
        steps: vec![*start],
    }));

    while let Some(Reverse(node)) = min_heap.pop() {
        cache.set(&node.position, node.step_count);
        if &node.position == end {
            return Some(node);
        }
        for add in DIRS.iter() {
            let next_pos = node.position.add(add);
            if is_valid(map, &next_pos) && cache.get_unsafe(&next_pos) > node.step_count + 1 {
                let mut steps: Vec<Pos> = node.steps.clone();
                steps.push(next_pos);
                min_heap.push(Reverse(Node {
                    step_count: node.step_count + 1,
                    position: next_pos,
                    steps,
                }));
            }
        }
    }

    None
}

fn part_1(input: &str) -> String {
    let (start, end, map) = parse(input);
    let mut cache = Map::from_value(usize::MAX, map.width, map.height);
    let path = find_path(&start, 0, &end, &map, &mut cache).unwrap();
    let path_len = path.step_count;
    println!("path_len: {}", path_len);
    let mut kwa: HashMap<usize, usize> = HashMap::new();
    for (steps, p) in path.steps.iter().enumerate() {
        for (d, d2) in DIR2S.iter().zip(DIRS.iter()) {
            let mut cache = cache.clone();
            let new_pos_short = p.add(d2);
            let new_pos = p.add(d);
            if map.is_within(&new_pos_short)
                && map.get_unsafe(&new_pos_short) == b'#'
                && is_valid(&map, &new_pos)
                && path_len - steps >= 2
            {
                let shorter = find_path(&new_pos, steps + 2, &end, &map, &mut cache);
                if let Some(shorter) = shorter {
                    if shorter.step_count > path_len {
                        println!(
                            "p: {p:?} {new_pos:?} it is longer!!!! shorter.len(): {}",
                            shorter.step_count
                        );
                    } else {
                        println!(
                            "p: {p:?} new_pos: {new_pos:?} :: len: {}",
                            shorter.step_count
                        );
                        kwa.entry(path_len - shorter.step_count)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for (k, v) in kwa.iter() {
        if k >= &100 {
            sum += *v;
        }
    }
    sum.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);
    println!("Part 1: {}", result);
}
