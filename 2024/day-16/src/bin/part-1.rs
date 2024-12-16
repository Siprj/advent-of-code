use core::panic;
use std::{cmp::Reverse, collections::BinaryHeap};

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

fn next_dir(dir_inc: usize, cost: usize, previous: &Node) -> Node {
    let next_dir = (previous.incoming_dir + dir_inc) % 4;
    Node {
        cost: previous.cost + cost,
        incoming_dir: next_dir,
        position: (
            DIRS[next_dir].0 + previous.position.0,
            DIRS[next_dir].1 + previous.position.1,
        ),
    }
}

fn get_edges(map: &Map<char>, node: &Node) -> Vec<Node> {
    let mut ret = Vec::new();
    let next_node = next_dir(0, 1, node);
    if *map_get(map, next_node.position) == '.' {
        ret.push(next_node)
    }
    let next_node = next_dir(1, 1001, node);
    if *map_get(map, next_node.position) == '.' {
        ret.push(next_node)
    }
    let next_node = next_dir(3, 1001, node);
    if *map_get(map, next_node.position) == '.' {
        ret.push(next_node)
    }
    let next_node = next_dir(2, 2001, node);
    if *map_get(map, next_node.position) == '.' {
        ret.push(next_node)
    }
    ret
}

fn find(map: &mut Map<char>) -> usize {
    let end: (i32, i32) = (1, (map[0].len() - 2) as i32);
    let start: (i32, i32) = ((map.len() - 2) as i32, 1);
    *map_get_mut(map, end) = '.';
    *map_get_mut(map, start) = '.';

    let mut map_cache: Vec<Vec<usize>> = (0..map.len())
        .map(|_| (0..map[0].len()).map(|_| usize::MAX).collect())
        .collect();
    *map_get_mut(&mut map_cache, start) = 0;
    let mut heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    heap.push(Reverse(Node {
        cost: 0,
        incoming_dir: 0,
        position: start,
    }));

    while let Some(Reverse(node)) = heap.pop() {
        if node.position == end {
            return node.cost;
        }

        if node.cost > *map_get(&map_cache, node.position) {
            continue;
        }

        for next_node in &get_edges(map, &node) {
            if next_node.cost < *map_get(&map_cache, next_node.position) {
                *map_get_mut(&mut map_cache, next_node.position) = next_node.cost;
                heap.push(Reverse(next_node.clone()));
            }
        }
    }
    panic!()
}

fn part_1(input: &str) -> String {
    let mut map = parse(input);
    find(&mut map).to_string()
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
        assert_eq!(part_1(input), "7036");
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
        assert_eq!(part_1(input), "11048");
    }
}
