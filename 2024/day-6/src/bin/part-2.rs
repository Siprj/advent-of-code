use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn next_direction(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn move_in_direction(dir: Direction, pos: &(i32, i32)) -> (i32, i32) {
    match dir {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
    }
}

fn position_is_outside(pos: &(i32, i32), size: &(i32, i32)) -> bool {
    pos.0 < 0 || pos.0 >= size.0 || pos.1 < 0 || pos.1 >= size.1
}

fn detect_loop(
    map: &[Vec<char>],
    size: &(i32, i32),
    extra_obsticle: (i32, i32),
    mut position: (i32, i32),
    mut direction: Direction,
) -> bool {
    let mut visited: HashSet<((i32, i32), Direction)> = HashSet::new();
    visited.insert((position, direction));
    loop {
        if let Some(next) = next_step(map, size, &position, direction, Some(extra_obsticle)) {
            position = next.0;
            direction = next.1;
            if !visited.insert((position, direction)) {
                return true;
            }
        } else {
            return false;
        }
    }
}

fn next_step(
    map: &[Vec<char>],
    size: &(i32, i32),
    position: &(i32, i32),
    mut direction: Direction,
    extra_position: Option<(i32, i32)>,
) -> Option<((i32, i32), Direction)> {
    let mut new_position = move_in_direction(direction, position);
    if position_is_outside(&new_position, size) {
        return None;
    }
    while map[new_position.0 as usize][new_position.1 as usize] == '#'
        || extra_position == Some(new_position)
    {
        direction = next_direction(direction);
        new_position = move_in_direction(direction, position);
        if position_is_outside(&new_position, size) {
            return None;
        }
    }
    Some((new_position, direction))
}

fn part_2(input: &str) -> String {
    let mut map = parse(input);
    let start_position: (i32, i32) = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .position(|c| *c == '^')
                .map(|x| (y as i32, x as i32))
        })
        .unwrap();
    let mut position = start_position;
    map[position.0 as usize][position.1 as usize] = '.';
    let mut direction = Direction::Up;
    let size = (map.len() as i32, map[0].len() as i32);

    let mut path: Vec<((i32, i32), Direction)> = Vec::new();
    while let Some(next) = next_step(&map, &size, &position, direction, None) {
        position = next.0;
        direction = next.1;
        path.push((position, direction));
    }

    let mut obsticles: HashSet<(i32, i32)> = HashSet::new();

    for p in path.iter() {
        if p.0 != start_position && detect_loop(&map, &size, p.0, start_position, Direction::Up){
            obsticles.insert(p.0);
        }
    }

    obsticles.len().to_string()
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
        let input: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part_2(input), "6");
    }
}
