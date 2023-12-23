use std::collections::HashSet;

struct Map(Vec<Vec<char>>);

const OFFSETS: [(isize,isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

const SLOPES: [(char, (isize, isize)); 4] = [('^', (0, -1)), ('>', (1, 0)), ('<', (-1, 0)), ('v', (0, 1))];

fn parse(input: &str) -> Map {
    Map(input.lines().map(|l| l.chars().collect()).collect())
}

impl Map {
    pub fn get(&self, x: isize, y: isize) -> char {
        self.0[y as usize][x as usize]
    }
    pub fn size(&self) -> (isize, isize) {
        (self.0[0].len() as isize, self.0.len() as isize)
    }

    fn walk_section(&self, mut x: isize, mut y: isize) -> ((isize, isize), isize) {
        let mut visited: HashSet<(isize,isize)> = HashSet::new();
        'l: loop {

            visited.insert((x,y));
            for (d_x, d_y) in OFFSETS {
                let next_x = x + d_x;
                let next_y = y + d_y;
                if next_x >= 0 && next_x < self.size().0 && next_y >= 0 && next_y < self.size().1 {
                    let next = self.get(next_x,next_y);
                    if next == '.' && !visited.contains(&(next_x, next_y)){
                        x = next_x;
                        y = next_y;
                        continue 'l;
                    }
                }
            }
            return ((x,y), visited.len() as isize);
        }
    }

    fn get_sloups(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        SLOPES.iter().filter_map(|(s, (x_d, y_d))| {
            let next = self.get(x + x_d, y + y_d);
            if &next == s {
                if x_d != &0 {
                    Some((x + (x_d*2), y))
                } else {
                    Some((x, y + (y_d*2)))
                }
            } else {
                None
            }
        }).collect()
    }
}

fn part_1(input: &str) -> String {
    let map = parse(input);
    let start: (isize, isize) = (1, 0);
    let end: (isize, isize) = (map.size().0 - 2isize, map.size().1 - 1isize);

    let mut stack: Vec<((isize, isize), isize)> = vec![(start, 0)];
    let mut max = 0;

    while let Some((section_start, count)) = stack.pop() {
        let (section_end, section_count) = map.walk_section(section_start.0, section_start.1);
        if section_end == end {
            max = max.max(count + section_count);
        } else {
            for next_section in map.get_sloups(section_end.0, section_end.1) {
                stack.push((next_section, count + section_count + 1));
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
        assert_eq!(part_1(input), "94");
    }
}
