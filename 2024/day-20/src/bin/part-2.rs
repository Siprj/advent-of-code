use std::cmp::{max, min};

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

fn find_path(end: Pos, map: &Map<u8>, distance_map: &mut Map<i32>) -> Vec<Pos> {
    let mut next_pos: Option<Pos> = Some(end);
    let mut path: Vec<Pos> = Vec::new();
    let mut cost = 0;

    while let Some(pos) = next_pos.take() {
        path.push(pos);
        distance_map.set(&pos, cost);
        for d in DIRS.iter() {
            let next = pos.add(d);
            if !is_valid(map, &next) {
                continue;
            }
            if let Some(c) = distance_map.get(&next) {
                if c == i32::MAX {
                    next_pos = Some(next);
                    break;
                }
            }
        }
        cost += 1;
    }

    path
}

fn cheats_for_position(
    cheat_distance: i32,
    pos: Pos,
    map: &Map<u8>,
    distances: &Map<i32>,
    cost_limit: i32,
) -> usize {
    let min_x = max(pos.x - cheat_distance, 1);
    let max_x = min(pos.x + cheat_distance, map.width - 2);
    let min_y = max(pos.y - cheat_distance, 1);
    let max_y = min(pos.y + cheat_distance, map.height - 2);
    let oritinal_cost = distances.get_unsafe(&pos);

    let mut cheats = 0;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let cheat_pos = Pos { x, y };
            let distance = (pos.x - x).abs() + (pos.y - y).abs();
            if distance > cheat_distance {
                continue;
            }
            if is_valid(map, &cheat_pos) {
                let cheat_end_cost = distances.get_unsafe(&cheat_pos);
                let cheated = cheat_end_cost - distance - oritinal_cost;
                if cheated >= cost_limit {
                    cheats += 1;
                }
            }
        }
    }

    cheats
}

fn part_2(input: &str) -> String {
    let (start, _end, map) = parse(input);
    let mut distances = Map::from_value(i32::MAX, map.width, map.height);
    let path = find_path(start, &map, &mut distances);

    let mut cheats = 0;
    for p in path {
        cheats += cheats_for_position(20, p, &map, &distances, 100);
    }

    cheats.to_string()
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
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(part_2(input), "6");
    }
}
