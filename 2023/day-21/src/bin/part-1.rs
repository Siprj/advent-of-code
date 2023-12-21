use std::collections::HashSet;

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
    next_positions: &mut HashSet<(isize, isize)>,
) {
    for next in NEXT {
        let next_x = next.0 + position.0;
        let next_y = next.1 + position.1;
        if next_x >= 0
            && next_x < map[0].len() as isize
            && next_y >= 0
            && next_y < map.len() as isize
            && map[next_y as usize][next_x as usize] != '#'
        {
            next_positions.insert((next_x, next_y));
        }
    }
}

fn part_1(input: &str, number_of_steps: u32) -> String {
    let (map, start) = parse(input);
    let mut positions: HashSet<(isize, isize)> = HashSet::from([start]);
    let map_size = map.len() * map[0].len();

    for _ in 0..number_of_steps {
        let mut new_positions = HashSet::with_capacity(map_size);
        for position in positions {
            valid_neighbor_positions(&map, &position, &mut new_positions);
        }
        positions = new_positions;
    }

    positions.len().to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input, 64);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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
        assert_eq!(part_1(input, 6), "16");
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
        assert_eq!(part_1(input, 100), "100");
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
        assert_eq!(part_1(input, 101), "101");
    }
    #[test]
    fn it_works_4() {
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
        assert_eq!(part_1(input, 102), "100");
    }

    #[test]
    fn it_works_5() {
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
        assert_eq!(part_1(input, 103), "101");
    }
}
