use std::collections::{HashMap, HashSet};

struct Map(Vec<Vec<char>>);

const OFFSETS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn parse(input: &str) -> Map {
    Map(input.lines().map(|l| l.chars().map(|c| if c != '#' {'.'} else {'#'}).collect()).collect())
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Node {
    edges: HashSet<(Pos, isize)>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Default)]
struct Pos {
    x: isize,
    y: isize,
}

impl Map {
    pub fn get(&self, x: isize, y: isize) -> char {
        self.0[y as usize][x as usize]
    }
    pub fn size(&self) -> (isize, isize) {
        (self.0[0].len() as isize, self.0.len() as isize)
    }

    pub fn get_nexts(&self, pos: &Pos, prev_pos: &Pos) -> Vec<Pos> {
        OFFSETS
            .iter()
            .filter_map(|(d_x, d_y)| {
                let next_x = pos.x + d_x;
                let next_y = pos.y + d_y;
                if next_x >= 0 && next_x < self.size().0 && next_y >= 0 && next_y < self.size().1 {
                    let next = self.get(next_x, next_y);
                    if next == '.' && !(next_x == prev_pos.x && next_y == prev_pos.y) {
                        Some(Pos {
                            x: next_x,
                            y: next_y,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    fn to_graph(&self, start: &Pos, end: &Pos) -> HashMap<Pos, Node> {
        let mut graph: HashMap<Pos, Node> = HashMap::new();
        let mut visited: HashSet<Pos> = HashSet::new();
        let mut stack: Vec<(Pos, Pos, isize, Pos)> = vec![(start.clone(), Pos::default(), 0, start.clone())];

        while let Some((current_pos, prev_pos, count, node)) = stack.pop() {
            if &current_pos == end {
                graph.entry(node.clone()).or_default().edges.insert((current_pos.clone(), count));
                graph.entry(current_pos.clone()).or_default().edges.insert((node.clone(), count));
            } else {
            let nexts = self.get_nexts(&current_pos, &prev_pos);
            if nexts.len() == 1 {
                stack.push((nexts[0].clone(), current_pos, count + 1, node));
            } else {
                graph.entry(node.clone()).or_default().edges.insert((current_pos.clone(), count));
                graph.entry(current_pos.clone()).or_default().edges.insert((node.clone(), count));
                if visited.insert(current_pos.clone()) {
                    for next in nexts {
                        stack.push((next, current_pos.clone(), 1, current_pos.clone()));
                    }
                }
            }
            }
        }
        graph
    }
}

fn part_1(input: &str) -> String {
    let map = parse(input);
    let start: Pos = Pos {x: 1, y:0};
    let end: Pos = Pos{ x: map.size().0 - 2isize, y: map.size().1 - 1isize};

    let mut max = 0;

    let graph = map.to_graph(&start, &end);
    let mut stack: Vec<(Pos, isize, HashSet<Pos>)> = vec![(start, 1, HashSet::new())];

    while let Some((current_pos, count, mut visited)) = stack.pop(){
        if current_pos == end {
            max = max.max(count);
        } else {
            visited.insert(current_pos.clone());
            for next_node in graph.get(&current_pos).unwrap().edges.iter() {
                if !visited.contains(&next_node.0) {
                    stack.push((next_node.0.clone(), next_node.1 + count, visited.clone()));
                }
            }
        }
    }

    (max - 1).to_string()
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
        let input: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(part_1(input), "154");
    }
}
