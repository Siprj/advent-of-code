use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (y.parse().unwrap(), x.parse().unwrap())
        })
        .collect()
}

#[derive(Debug)]
struct Node {
    x: i32,
    y: i32,
    count: usize,
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.count.eq(&other.count)
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
        self.count.cmp(&other.count)
    }
}

fn is_valid(map: &[Vec<bool>], (y, x): (i32, i32)) -> bool {
    let width = map[0].len();
    let height = map.len();

    if y < 0 || y >= height as i32 || x < 0 || x >= width as i32 {
        return false;
    }

    !map[y as usize][x as usize]
}

fn get_nexts(map: &[Vec<bool>], cache: &mut HashSet<(i32, i32)>, node: &Node) -> Vec<Node> {
    let mut ret = Vec::new();
    let next_y = node.y + 1;
    let next_x = node.x;
    let next_node = Node {
        x: next_x,
        y: next_y,
        count: node.count + 1,
    };
    if is_valid(map, (next_y, next_x)) && !cache.contains(&(next_y, next_x)) {
        cache.insert((next_y, next_x));
        ret.push(next_node)
    }
    let next_y = node.y - 1;
    let next_x = node.x;
    let next_node = Node {
        x: next_x,
        y: next_y,
        count: node.count + 1,
    };
    if is_valid(map, (next_y, next_x)) && !cache.contains(&(next_y, next_x)) {
        cache.insert((next_y, next_x));
        ret.push(next_node)
    }
    let next_y = node.y;
    let next_x = node.x + 1;
    let next_node = Node {
        x: next_x,
        y: next_y,
        count: node.count + 1,
    };
    if is_valid(map, (next_y, next_x)) && !cache.contains(&(next_y, next_x)) {
        cache.insert((next_y, next_x));
        ret.push(next_node)
    }
    let next_y = node.y;
    let next_x = node.x - 1;
    let next_node = Node {
        x: next_x,
        y: next_y,
        count: node.count + 1,
    };
    if is_valid(map, (next_y, next_x)) && !cache.contains(&(next_y, next_x)) {
        cache.insert((next_y, next_x));
        ret.push(next_node)
    }
    ret
}

fn part_1(input: &str, width: i32, height: i32, iterations: usize) -> String {
    let falling = parse(input);
    let mut map: Vec<Vec<bool>> = (0..height)
        .map(|_| (0..width).map(|_| false).collect())
        .collect();
    for f in falling.iter().take(iterations) {
        map[f.0 as usize][f.1 as usize] = true;
    }

    let mut queue: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    queue.push(Reverse(Node {
        x: 0,
        y: 0,
        count: 0,
    }));
    let mut cache = HashSet::new();
    while let Some(Reverse(node)) = queue.pop() {
        if node.x == width - 1 && node.y == height - 1 {
            return node.count.to_string();
        }
        for n in get_nexts(&map, &mut cache, &node) {
            queue.push(Reverse(n));
        }
    }

    panic!("not cool");
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input, 71, 71, 1024);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(part_1(input, 7, 7, 12), "22");
    }
}
