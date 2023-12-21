use std::collections::{HashMap, HashSet, VecDeque};

use ndarray::{array, Array1, Array2};
use ndarray_linalg::Solve;

fn parse(input: &str) -> (Vec<Vec<char>>, (isize, isize)) {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.iter().position(|c| c == &'S').map(|x| (x, y)))
        .unwrap();
    map[start.1][start.0] = '.';
    (map, (start.0 as isize, start.1 as isize))
}

const NEXT: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn valid_neighbor_positions(
    map: &Vec<Vec<char>>,
    position: &(isize, isize),
    modulo: bool,
) -> [Option<(isize, isize)>; 4] {
    let mut out = [None, None, None, None];
    for (i, next) in NEXT.iter().enumerate() {
        let next_x = next.0 + position.0;
        let next_y = next.1 + position.1;
        if modulo {
            if map[next_y.rem_euclid(map.len() as isize) as usize]
                [next_x.rem_euclid(map[0].len() as isize) as usize]
                != '#'
            {
                out[i] = Some((next_x, next_y));
            }
        } else if next_x >= 0
            && next_x < map[0].len() as isize
            && next_y >= 0
            && next_y < map.len() as isize
            && map[next_y as usize][next_x as usize] != '#'
        {
            out[i] = Some((next_x, next_y));
        }
    }
    out
}

fn count(map: &Vec<Vec<char>>, start: (isize, isize), number_of_steps: u64) -> usize {
    let map_size = map.len() * map[0].len();
    let mut positions: HashSet<(isize, isize)> = HashSet::from([start]);

    for _ in 0..number_of_steps {
        let mut new_positions = HashSet::with_capacity(map_size);
        for position in positions {
            for next_pos in valid_neighbor_positions(map, &position, true)
                .into_iter()
                .flatten()
            {
                new_positions.insert(next_pos);
            }
        }
        positions = new_positions;
    }

    positions.len()
}

fn part_2(input: &str, number_of_steps: usize) -> String {
    let (map, start) = parse(input);

    // Important observations:
    //  1. The row and column of the start positions are empty. This means the
    //  movement from one cell to anouther is multiple of the map size.
    //  2. The corners are the same distance from the center. And it is the
    //  shortest path possible on empty map.
    //  3. The (number_of_steps - map.size/2) / map.size is a nice round number.
    let vector = Array1::from_iter([65, 196, 327u64].iter().map(|steps| {
        let c = count(&map, start, *steps);
        println!(
            "start: ({},{}), steps: {steps}, count: {c}",
            start.0, start.1
        );
        c as f64
    }));

    let cell_steps: usize = (number_of_steps - (map.len() / 2)) / map.len();
    let mat: Array2<f64> = array![[0., 0., 1.], [1., 1., 1.], [4., 2., 1.]];
    let quad_parts = mat.solve(&vector).unwrap();
    let result = cell_steps * cell_steps * (quad_parts[0] as usize)
        + cell_steps * (quad_parts[1] as usize)
        + (quad_parts[2] as usize);
    result.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input, 26501365);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let input: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part_2(input, 6), "16");
    }

    #[test]
    fn it_works_2() {
        let input: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part_2(input, 10), "50");
    }
    #[test]
    fn it_works_3() {
        let input: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part_2(input, 50), "1594");
    }
}
