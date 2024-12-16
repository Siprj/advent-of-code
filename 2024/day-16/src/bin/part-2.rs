use core::panic;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

type Map<T> = Vec<Vec<T>>;

fn parse(input: &str) -> Map<char> {
    input.trim().lines().map(|l| l.chars().collect()).collect()
}

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Clone)]
struct Node {
    cost: usize,
    incoming_dir: usize,
    position: (i32, i32),
    steps: Vec<(i32, i32)>,
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

fn map_get<T>(map: &Map<T>, (y, x): (i32, i32)) -> &T {
    &map[y as usize][x as usize]
}

fn map_get_mut<T>(map: &mut Map<T>, (y, x): (i32, i32)) -> &mut T {
    &mut map[y as usize][x as usize]
}

// This is overcomplicated and done after 24hours without sleep......
fn next_dir(
    map: &Map<char>,
    cache: &mut HashMap<(i32, i32, usize), usize>,
    dir_inc: usize,
    cost: usize,
    previous: &Node,
) -> Option<Node> {
    let next_dir = (previous.incoming_dir + dir_inc) % 4;
    let next_position = (
        DIRS[next_dir].0 + previous.position.0,
        DIRS[next_dir].1 + previous.position.1,
    );
    let next_cost = previous.cost + cost;
    if *map_get(map, next_position) != '.' {
        return None;
    }
    match cache.entry((next_position.0, next_position.1, next_dir)) {
        std::collections::hash_map::Entry::Occupied(mut entry) => {
            if next_cost > *entry.get() {
                return None;
            } else {
                *entry.get_mut() = next_cost;
            }
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            entry.insert(next_cost);
        }
    }

    let mut steps = previous.steps.clone();
    steps.push(next_position);
    Some(Node {
        cost: next_cost,
        steps,
        incoming_dir: next_dir,
        position: next_position,
    })
}

fn get_edges(
    map: &Map<char>,
    cache: &mut HashMap<(i32, i32, usize), usize>,
    node: &Node,
) -> Vec<Node> {
    let mut ret = Vec::new();
    if let Some(next_node) = next_dir(map, cache, 0, 1, node) {
        ret.push(next_node)
    }
    if let Some(next_node) = next_dir(map, cache, 1, 1001, node) {
        ret.push(next_node)
    }
    if let Some(next_node) = next_dir(map, cache, 3, 1001, node) {
        ret.push(next_node)
    }
    if let Some(next_node) = next_dir(map, cache, 2, 2001, node) {
        ret.push(next_node)
    }
    ret
}

fn find(map: &mut Map<char>) -> Vec<Node> {
    let end: (i32, i32) = (1, (map[0].len() - 2) as i32);
    let start: (i32, i32) = ((map.len() - 2) as i32, 1);
    *map_get_mut(map, end) = '.';
    *map_get_mut(map, start) = '.';

    let mut map_cache: Vec<Vec<usize>> = (0..map.len())
        .map(|_| (0..map[0].len()).map(|_| usize::MAX).collect())
        .collect();
    *map_get_mut(&mut map_cache, start) = 0;
    let mut heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    let mut steps: Vec<(i32, i32)> = Vec::with_capacity(300);
    steps.push(start);
    heap.push(Reverse(Node {
        cost: 0,
        incoming_dir: 0,
        position: start,
        steps,
    }));

    let mut best_cost: Option<usize> = None;
    let mut best_paths: Vec<Node> = Vec::new();
    let mut cache: HashMap<(i32, i32, usize), usize> = HashMap::new();

    while let Some(Reverse(node)) = heap.pop() {
        if best_cost.is_none() && node.position == end {
            best_cost = Some(node.cost);
            best_paths.push(node.clone());
        } else if Some(node.cost) == best_cost && node.position == end {
            best_paths.push(node.clone());
        } else if best_cost.is_some() && Some(node.cost) > best_cost {
            return best_paths;
        }

        if node.cost > *map_get(&map_cache, node.position) {
            continue;
        }

        for next_node in &get_edges(map, &mut cache, &node) {
            heap.push(Reverse(next_node.clone()));
        }
    }
    panic!()
}

fn part_2(input: &str) -> String {
    let mut map = parse(input);
    let paths = find(&mut map);
    println!("paths: {}", paths.len());
    let spots: HashSet<(i32, i32)> = paths.iter().flat_map(|n| n.steps.clone()).collect();
    spots.len().to_string()
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
        let input: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(part_2(input), "45");
    }

    #[test]
    fn it_works2() {
        let input: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(part_2(input), "64");
    }
}
