use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, (i32, i32)) {
    input
        .lines()
        .enumerate()
        .fold((HashMap::new(), (0, 0)), |mut acc, (y, l)| {
            for (x, c) in l.chars().enumerate() {
                if c.is_ascii_alphanumeric() {
                    acc.0
                        .entry(c)
                        .and_modify(|e: &mut Vec<(i32, i32)>| e.push((x as i32, y as i32)))
                        .or_insert(vec![(x as i32, y as i32)]);
                }
            }
            (acc.0, (l.len() as i32, y as i32 + 1))
        })
}

fn fitts_in(pos: &(i32, i32), size: &(i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < size.0 && pos.1 >= 0 && pos.1 < size.1
}

fn part_2(input: &str) -> String {
    let (map, size) = parse(input);
    let mut unique_locations = HashSet::new();
    for antenas in map.values() {
        if antenas.len() < 2 {
            continue;
        }
        for (a_1, a_2) in antenas.iter().tuple_combinations() {
            let distance = (a_1.0 - a_2.0, a_1.1 - a_2.1);

                unique_locations.insert(*a_1);
                unique_locations.insert(*a_2);
            for i in 1.. {
                let antinode_1 = (a_1.0 + (distance.0 * i), a_1.1 + (distance.1 * i));
                if fitts_in(&antinode_1, &size) {
                    unique_locations.insert(antinode_1);
                } else {
                    break;
                }
            }
            for i in 1.. {
                let antinode_2 = (a_2.0 - (distance.0 * i), a_2.1 - (distance.1 * i));
                if fitts_in(&antinode_2, &size) {
                    unique_locations.insert(antinode_2);
                } else {
                    break;
                }
            }
        }
    }

    unique_locations.len().to_string()
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
        let input: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part_2(input), "34");
    }
}
