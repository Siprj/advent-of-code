use std::collections::HashSet;

use winnow::{
    ascii::{multispace0, multispace1, dec_int},
    combinator::{eof, repeat_till0, terminated},
    token::{one_of, take_until1},
    PResult, Parser,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    U,
    D,
    R,
    L,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Line {
    direction: Direction,
    steps: i32,
}

fn direction_parser(input: &mut &str) -> PResult<Direction> {
    let d = one_of(['U', 'D', 'R', 'L']).parse_next(input)?;
    Ok(match d {
        'U' => Direction::U,
        'D' => Direction::D,
        'L' => Direction::L,
        'R' => Direction::R,
        _ => unreachable!(),
    })
}


fn line_parser(input: &mut &str) -> PResult<Line> {
    let hex_parse = terminated('(', terminated(take_until1(")"),")"));

    let (direction, steps) = (
        terminated(direction_parser, multispace1),
        terminated(dec_int, terminated(multispace1, hex_parse)),
    )
        .parse_next(input)?;
    Ok(Line {
        direction,
        steps,
    })
}

fn parse(input: &str) -> Vec<Line> {
    repeat_till0(terminated(line_parser, multispace0), eof)
        .parse(input)
        .unwrap()
        .0
}

fn walk(lines: &Vec<Line>) -> Vec<(i32, i32)> {
    let mut trench = vec![];
    let mut pos = (0i32, 0i32);
    for line in lines {
        match line.direction {
            Direction::U => {
                for diff in 1..=line.steps {
                    trench.push((pos.0, pos.1 - diff));
                }
            },
            Direction::D => {
                for diff in 1..=line.steps {
                    trench.push((pos.0, pos.1 + diff));
                }
            }
            Direction::R => {
                for diff in 1..=line.steps {
                    trench.push((pos.0 + diff, pos.1));
                }
            }
            Direction::L => {
                for diff in 1..=line.steps {
                    trench.push((pos.0 - diff, pos.1));
                }
            }
        }
        pos = trench.last().unwrap().clone();
    }
    trench
}

fn trace_iside(trench_set: &HashSet<(i32, i32)>, pos: &(i32, i32), min_x: i32) -> bool {
    let mut intersections = 0;
    for x in min_x..pos.0 {
        if trench_set.contains(&(x, pos.1)) {
            intersections += 1;
        }
    }
    intersections % 2 == 1
}


fn part_1(input: &str) -> String {
    let lines = parse(input);
    let trench = walk(&lines);
    let mut trench_set: HashSet<(i32, i32)> = trench.iter().copied().collect();
    let min_x = trench.iter().min_by_key(|a| a.0).unwrap().0;

    let start_positions = vec![(1,1), (-1,-1), (-1,1), (1, -1)];
    let start = start_positions.iter().find(|pos|trace_iside(&trench_set, pos, min_x)).unwrap().clone();


    let mut stack: Vec<(i32, i32)> = vec![start];

    while let Some(pos) = stack.pop() {
        let new_pos = (pos.0 - 1, pos.1);
        if !trench_set.contains(&new_pos) {
            trench_set.insert(new_pos);
            stack.push(new_pos);
        }
        let new_pos = (pos.0 + 1, pos.1);
        if !trench_set.contains(&new_pos) {
            trench_set.insert(new_pos);
            stack.push(new_pos);
        }
        let new_pos = (pos.0, pos.1 - 1);
        if !trench_set.contains(&new_pos) {
            trench_set.insert(new_pos);
            stack.push(new_pos);
        }
        let new_pos = (pos.0, pos.1 + 1);
        if !trench_set.contains(&new_pos) {
            trench_set.insert(new_pos);
            stack.push(new_pos);
        }
    }

//    for y in min_y..=max_y {
//        for x in min_x..=max_x {
//            if trench_set.contains(&(x,y)) {
//                print!("#");
//            } else {
//                print!(".");
//            }
//        }
//        println!();
//    }

    trench_set.len().to_string()
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
        let input: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(part_1(input), "62");
    }

    #[test]
    fn it_works_2() {
        let input: &str = "R 6 (#70c710)";
        assert_eq!(
            line_parser.parse(input).unwrap(),
            Line {
                direction: Direction::R,
                steps: 6,
            }
        );
    }
}
