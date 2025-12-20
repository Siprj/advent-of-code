use std::collections::HashSet;

use day_7::parse;

type Beams = HashSet<(usize, usize)>;
type Grid = Vec<Vec<char>>;

fn part_1(input: &str) -> String {
    let (grid, start) = parse(input);
    let mut beams: Beams = HashSet::new();
    beams.insert((start, 0));
    let mut count: usize = 0;

    loop {
        let (next_beams, c) = step_beams_and_count(&beams, &grid);
        count += c;
        if next_beams.is_empty() {
            break;
        }
        beams = next_beams;
    }

    count.to_string()
}

fn step_beams_and_count(beams: &Beams, grid: &Grid) -> (Beams, usize) {
    let mut new_beams: Beams = HashSet::new();
    let mut count = 0;

    for (x, y) in beams.iter().copied() {
        let next_y = y + 1;
        if next_y < grid.len() {
            if grid[next_y][x] == '^' {
                count += 1;
                new_beams.insert((x + 1, next_y));
                new_beams.insert((x - 1, next_y));
            } else {
                new_beams.insert((x, next_y));
            }
        }
    }
    (new_beams, count)
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
        let input: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part_1(input), "21");
    }
}
