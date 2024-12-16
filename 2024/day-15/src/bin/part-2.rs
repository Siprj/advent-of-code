use itertools::Itertools;

type Map = Vec<Vec<char>>;

fn parse_map(str: &str) -> (Map, (i32, i32)) {
    let mut map: Map = str
        .lines()
        .skip(1)
        .map(|l| {
            let mut l = l.chars().skip(1).collect_vec();
            l.truncate(l.len() - 1);
            l.iter()
                .copied()
                .flat_map(|c| match c {
                    'O' => vec!['[', ']'],
                    '@' => vec!['@', '.'],
                    _ => vec![c, c],
                })
                .collect_vec()
        })
        .collect_vec();
    map.truncate(map.len() - 1);
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, c)| {
                if *c == '@' {
                    Some((y as i32, x as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();
    map[start.0 as usize][start.1 as usize] = '.';
    (map, start)
}

fn parse(input: &str) -> (Map, (i32, i32), Vec<char>) {
    let (map_str, moves_str) = input.trim().split_once("\n\n").unwrap();
    let (mut map, start) = parse_map(map_str);
    let moves: Vec<char> = moves_str.lines().flat_map(|l| l.chars()).collect_vec();
    map[start.0 as usize][start.1 as usize] = '.';
    (map, start, moves)
}

fn test_push(map: &Map, (y, x): (i32, i32), (dy, dx): (i32, i32)) -> bool {
    if y < 0
        || y >= map.len() as i32
        || x < 0
        || x >= map[0].len() as i32
        || map[y as usize][x as usize] == '#'
    {
        return false;
    } else if map[y as usize][x as usize] == '.' {
        return true;
    }
    let ny = y + dy;
    let nx = x + dx;
    if map[y as usize][x as usize] == '[' {
        test_push(map, (ny, nx), (dy, dx))
            && if dy != 0 {
                test_push(map, (ny, nx + 1), (dy, dx))
            } else {
                true
            }
    } else if map[y as usize][x as usize] == ']' {
        test_push(map, (ny, nx), (dy, dx))
            && if dy != 0 {
                test_push(map, (ny, nx - 1), (dy, dx))
            } else {
                true
            }
    } else {
        false
    }
}

fn push(map: &mut Map, c: char, is_second: bool, (y, x): (i32, i32), (dy, dx): (i32, i32)) {
    let ny = y + dy;
    let nx = x + dx;

    if map[y as usize][x as usize] == '[' {
        push(map, '[', false, (ny, nx), (dy, dx));
        if dy != 0 && !is_second {
            push(map, '.', true, (y, nx + 1), (dy, dx))
        }
    } else if map[y as usize][x as usize] == ']' {
        push(map, ']', false, (ny, nx), (dy, dx));
        if dy != 0 && !is_second {
            push(map, '.', true, (y, nx - 1), (dy, dx))
        }
    }
    map[y as usize][x as usize] = c;
}

fn in_direction(map: &mut Map, (y, x): (i32, i32), (dy, dx): (i32, i32)) -> (i32, i32) {
    let ny = y + dy;
    let nx = x + dx;
    if ny < 0
        || ny >= map.len() as i32
        || nx < 0
        || nx >= map[0].len() as i32
        || map[ny as usize][nx as usize] == '#'
    {
        return (y, x);
    }
    if map[ny as usize][nx as usize] == '.' {
        (ny, nx)
    } else if test_push(map, (ny, nx), (dy, dx)) {
        push(map, '.', false, (ny, nx), (dy, dx));
        map[ny as usize][nx as usize] = '.';
        (ny, nx)
    } else {
        (y, x)
    }
}

fn step(map: &mut Map, start: (i32, i32), c: char) -> (i32, i32) {
    match c {
        '>' => in_direction(map, start, (0, 1)),
        '<' => in_direction(map, start, (0, -1)),
        '^' => in_direction(map, start, (-1, 0)),
        'v' => in_direction(map, start, (1, 0)),
        _ => {
            panic!("unknown move");
        }
    }
}

fn part_1(input: &str) -> String {
    let (mut map, mut pos, moves) = parse(input);
    for m in moves.into_iter() {
        pos = step(&mut map, pos, m);
    }
    map.into_iter()
        .enumerate()
        .map(|(y, l)| {
            let y_dist = 100 * (y as i32 + 1);
            l.into_iter()
                .enumerate()
                .filter(|(_, c)| *c == '[')
                .map(|(x, _)| x as i32 + 2 + y_dist)
                .sum::<i32>()
        })
        .sum::<i32>()
        .to_string()
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
        let input: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(part_1(input), "9021");
    }
}
