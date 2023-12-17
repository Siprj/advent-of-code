#![feature(binary_heap_into_iter_sorted)]
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::usize;

const DIRECTION_LIMIT: u32 = 2;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
enum Direction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: u32,
    position: (usize, usize),
    direction: Direction,
//    path: Vec<(usize, usize, u32)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| {
            self.position
                .cmp(&other.position)
                .then_with(|| self.direction.cmp(&other.direction))
        })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(map: &Vec<Vec<u32>>, goal: (usize, usize)) -> u32 {
    let mut visited: HashMap<(usize, usize, Direction), u32> = HashMap::new();

    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        position: (0, 0),
        direction: Direction::Right(0),
//        path: vec![(0, 0, 0)],
    });
    heap.push(State {
        cost: 0,
        position: (0, 0),
        direction: Direction::Down(0),
//        path: vec![(0, 0, 0)],
    });


    let mut count: usize = 0;


    while let Some(best) = heap.pop() {
        if count % 10000 == 0 {
            println!("best: {:?}", best);
            println!("heap.len(): {}", heap.len());
            heap = heap.into_iter_sorted().take(10000000).collect();
            println!("visited.len(): {}", visited.len());
        }
        count += 1;
        //        println!("best: {best:?}");
        //        println!("heap: {heap:?}\n");
        // Alternatively we could have continued to find all shortest paths
        if best.position == goal {
            //for y in 0..map.len() {
            //    for x in 0..map[0].len() {
            //        //print!("({:>2}, {:>2}) [{}]", x, y, map[y][x]);
            //        print!(" [{}]",map[y][x]);
            //        if visited[y][x] == u32::MAX {
            //            print!(" MAX ");
            //        } else {

            //            print!("{:>4} ", visited[y][x])
            //        }
            //    }
            //    println!();
            //}
            //println!("best: {:?}", best);
            return best.cost;
        }

        // Important as we may have already found a better way
        if let Some(visited_cost) = visited.get(&(best.position.0, best.position.1, best.direction)) {
            if visited_cost < &best.cost {
            continue;
            } else {
                visited.insert((best.position.0, best.position.1, best.direction), best.cost);
            }
        } else {
            visited.insert((best.position.0, best.position.1, best.direction), best.cost);
        }

        match best.direction {
            Direction::Up(n) => {
                if n != DIRECTION_LIMIT {
                    if let Some(next_y) = best.position.1.checked_sub(1) {
//                        let mut new_path = best.path.clone();
                        let cost = map[next_y][best.position.0] + best.cost;
//                        new_path.push((best.position.0,next_y, cost));
                        let next = State {
                            cost,
                            position: (best.position.0, next_y),
                            direction: Direction::Up(n + 1),
//                            path: new_path,
                        };
                        heap.push(next);
                    }
                }
                if let Some(next_x) = best.position.0.checked_sub(1) {
                    let cost = map[best.position.1][next_x] + best.cost;
//                    let mut new_path = best.path.clone();
//                    new_path.push((next_x, best.position.1, cost));
                    let next = State {
                        cost,
                        position: (next_x, best.position.1),
                        direction: Direction::Left(0),
//                        path: new_path,
                    };
                    heap.push(next);
                }
                let next_x = best.position.0 + 1;
                if next_x < map[0].len() {
                    let cost = map[best.position.1][next_x] + best.cost;
//                    let mut new_path = best.path.clone();
//                    new_path.push((next_x, best.position.1, cost));
                    let next = State {
                        cost,
                        position: (next_x, best.position.1),
                        direction: Direction::Right(0),
//                        path: new_path,
                    };
                    heap.push(next);
                }
            }
            Direction::Down(n) => {
                if n != DIRECTION_LIMIT {
                    let next_y = best.position.1 + 1;
                    if next_y < map.len() {
//                        let mut new_path = best.path.clone();
                        let cost = map[next_y][best.position.0] + best.cost;
//                        new_path.push((best.position.0, next_y, cost));
                        let next = State {
                            cost,
                            position: (best.position.0, next_y),
                            direction: Direction::Down(n + 1),
//                            path: new_path,
                        };
                        heap.push(next);
                    }
                }
                if let Some(next_x) = best.position.0.checked_sub(1) {
                    let cost = map[best.position.1][next_x] + best.cost;
//                    let mut new_path = best.path.clone();
//                    new_path.push((next_x, best.position.1, cost));
                    let next = State {
                        cost,
                        position: (next_x, best.position.1),
                        direction: Direction::Left(0),
//                        path: new_path,
                    };
                    heap.push(next);
                }
                let next_x = best.position.0 + 1;
                if next_x < map[0].len() {
                    let cost = map[best.position.1][next_x] + best.cost;
//                    let mut new_path = best.path.clone();
//                    new_path.push((next_x, best.position.1, cost));
                    let next = State {
                        cost,
                        position: (next_x, best.position.1),
                        direction: Direction::Right(0),
//                        path: new_path,
                    };
                    heap.push(next);
                }
            }
            Direction::Left(n) => {
                if n != DIRECTION_LIMIT {
                    if let Some(next_x) = best.position.0.checked_sub(1) {
//                        let mut new_path = best.path.clone();
                        let cost = map[best.position.1][next_x] + best.cost;
//                        new_path.push((next_x, best.position.1, cost));
                        let next = State {
                            cost,
                            position: (next_x, best.position.1),
                            direction: Direction::Left(n + 1),
//                            path: new_path,
                        };
                        heap.push(next);
                    }
                }
                if let Some(next_y) = best.position.1.checked_sub(1) {
                    let cost = map[next_y][best.position.0] + best.cost;
//                    let mut new_path = best.path.clone();
//                    new_path.push((best.position.0, next_y, cost));
                    let next = State {
                        cost,
                        position: (best.position.0, next_y),
                        direction: Direction::Up(0),
//                        path: new_path,
                    };
                    heap.push(next);
                }
                let next_y = best.position.1 + 1;
                if next_y < map.len() {
                    let cost = map[next_y][best.position.0] + best.cost;
//                    let mut new_path = best.path.clone();
//                    new_path.push((best.position.0, next_y, cost));
                    let next = State {
                        cost,
                        position: (best.position.0, next_y),
                        direction: Direction::Down(0),
//                        path: new_path,
                    };
                    heap.push(next);
                }
            }

            Direction::Right(n) => {
                if n != DIRECTION_LIMIT {
                    let next_x = best.position.0 + 1;
                    if next_x < map[0].len() {
//                        let mut new_path = best.path.clone();
                        let cost = map[best.position.1][next_x] + best.cost;
//                        new_path.push((next_x, best.position.1, cost));
                        let next = State {
                            cost,
                            position: (next_x, best.position.1),
                            direction: Direction::Right(n + 1),
//                            path: new_path,
                        };
                        heap.push(next);
                    }
                }
                if let Some(next_y) = best.position.1.checked_sub(1) {
                    let cost = map[next_y][best.position.0] + best.cost;
//                    let mut new_path = best.path.clone();
//                    new_path.push((best.position.0, next_y, cost));
                    let next = State {
                        cost,
                        position: (best.position.0, next_y),
                        direction: Direction::Up(0),
//                        path: new_path,
                    };
                    heap.push(next);
                }
                let next_y = best.position.1 + 1;
                if next_y < map.len() {
                    let cost = map[next_y][best.position.0] + best.cost;
//                    let mut new_path = best.path.clone();
//                    new_path.push((best.position.0, next_y, cost));
                    let next = State {
                        cost,
                        position: (best.position.0, next_y),
                        direction: Direction::Down(0),
//                        path: new_path,
                    };
                    heap.push(next);
                }
            }
        }
    }
    unreachable!()
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_1(input: &str) -> String {
    let map = parse(input);
    shortest_path(&map, (map[0].len() - 1, map.len() - 1)).to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(part_1(input), "102");
    }
}
