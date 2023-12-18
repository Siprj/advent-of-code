
use winnow::{
    ascii::multispace0,
    combinator::{eof, repeat_till0, terminated, preceded},
    token::{one_of, take_until1, take_while},
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
    steps: isize,
}

fn direction_parser(input: &mut &str) -> PResult<Direction> {
    let d = one_of(['0', '1', '2', '3']).parse_next(input)?;
    Ok(match d {
        '0' => Direction::R,
        '1' => Direction::D,
        '2' => Direction::L,
        '3' => Direction::U,
        _ => unreachable!(),
    })
}

fn skip_beginning(input: &mut &str) -> PResult<()> {
    terminated(take_until1("#"), "#").parse_next(input).map(|_| ())
}

fn get_steps(input: &mut &str) -> PResult<isize>{
    take_while(5, |c: char| c.is_ascii_hexdigit())
        .try_map(|input| isize::from_str_radix(input, 16))
        .parse_next(input)
}

fn line_parser(input: &mut &str) -> PResult<Line> {
    let (steps, direction) = preceded(skip_beginning, (get_steps, terminated(direction_parser, ")"))).parse_next(input)?;
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


fn trench_vertices(lines: &Vec<Line>) -> (Vec<(isize, isize)>, isize) {
    let mut trench = vec![];
    let mut pos = (0isize, 0isize);
    let mut trench_count = 0;
    for line in lines {
        match line.direction {
            Direction::U => {
                trench_count += line.steps;
                trench.push((pos.0, pos.1 - line.steps));
            },
            Direction::D => {
                trench_count += line.steps;
                trench.push((pos.0, pos.1 + line.steps));
            }
            Direction::R => {
                trench_count += line.steps;
                trench.push((pos.0 + line.steps, pos.1));
            }
            Direction::L => {
                trench_count += line.steps;
                trench.push((pos.0 - line.steps, pos.1));
            }
        }
        pos = trench.last().unwrap().clone();
    }
    (trench, trench_count)
}

fn part_2(input: &str) -> String {
    let lines = parse(input);
    let (trench, trench_count) = trench_vertices(&lines);


    let mut area = 0isize;

    for i in 0..trench.len() {
        let i = i as isize;
        let ii = (i + 1) % (trench.len() as isize);
        let i_ = (i - 1).rem_euclid(trench.len() as isize);
        area += trench[i as usize].1 *(trench[i_ as usize].0 - trench[ii as usize].0);
    }

    ((area.abs() / 2) + (trench_count/2 ) + 1).to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);
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
        assert_eq!(part_2(input), "952408144115");
    }

    #[test]
    fn it_works_2() {
        let input: &str = "R 6 (#70c710)";
        assert_eq!(
            line_parser.parse(input).unwrap(),
            Line {
                direction: Direction::R,
                steps: 461937,
            }
        );
    }
}
