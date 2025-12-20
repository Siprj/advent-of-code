use std::collections::HashMap;

use day_7::parse;

type Grid = Vec<Vec<char>>;
type Cache = HashMap<(usize, usize), usize>;

fn part_2(input: &str) -> String {
    let (grid, start) = parse(input);
    let count = step(&grid, (start, 0), &mut HashMap::new());

    count.to_string()
}

fn beam_hit(grid: &Grid, beam_start: (usize, usize)) -> Option<usize> {
    (beam_start.1..grid.len()).find(|&y| grid[y][beam_start.0] == '^')
}

fn step(grid: &Grid, beam_start: (usize, usize), cache: &mut Cache) -> usize {
    if let Some(v) = cache.get(&beam_start) {
        *v
    } else if let Some(splitter_y) = beam_hit(grid, beam_start) {
        let sum = step(grid, (beam_start.0 + 1, splitter_y), cache)
            + step(grid, (beam_start.0 - 1, splitter_y), cache);
        cache.insert(beam_start, sum);
        sum
    } else {
        1
    }
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
        assert_eq!(part_2(input), "40");
    }
}
