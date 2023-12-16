use std::{cmp::max, collections::HashSet};

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

type Grid = Vec<Vec<char>>;

fn width(grid: &Grid) -> usize {
    grid[0].len()
}

fn height(grid: &Grid) -> usize {
    grid.len()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_in_direction(&self, grid: &Grid, pos: &(usize, usize)) -> Option<(usize, usize)> {
        match self {
            Up => {
                if pos.1 == 0 {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                }
            }
            Down => {
                if pos.1 == height(grid) - 1 {
                    None
                } else {
                    Some((pos.0, pos.1 + 1))
                }
            }
            Left => {
                if pos.0 == 0 {
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                }
            }
            Right => {
                if pos.0 == width(grid) - 1 {
                    None
                } else {
                    Some((pos.0 + 1, pos.1))
                }
            }
        }
    }
}

use Direction::*;

fn part_2(input: &str) -> String {
    let grid = parse(input);
    let mut walked: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut beams: Vec<(usize, usize, Direction)> = vec![];
    let mut max_energy = 0;
    let start_positions: Vec<(usize, usize, Direction)> = (0..width(&grid))
        .map(|x| (x, 0, Down))
        .chain((0..width(&grid)).map(|x| (x, height(&grid) - 1, Down)))
        .chain((0..height(&grid)).map(|y| (0, y, Right)))
        .chain((0..height(&grid)).map(|y| (width(&grid) - 1, y, Left)))
        .collect();

    for start_pos in start_positions {
        beams.push(start_pos);
        walked.clear();
        while let Some((x, y, direction)) = beams.pop() {
            if walked.insert((x, y, direction)) {
                match grid[y][x] {
                    '.' => {
                        if let Some((new_x, new_y)) = direction.move_in_direction(&grid, &(x, y)) {
                            beams.push((new_x, new_y, direction));
                        }
                    }
                    '/' => match direction {
                        Up => {
                            if let Some((new_x, new_y)) = Right.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Right));
                            }
                        }
                        Down => {
                            if let Some((new_x, new_y)) = Left.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Left));
                            }
                        }
                        Left => {
                            if let Some((new_x, new_y)) = Down.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Down));
                            }
                        }
                        Right => {
                            if let Some((new_x, new_y)) = Up.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Up));
                            }
                        }
                    },
                    '\\' => match direction {
                        Up => {
                            if let Some((new_x, new_y)) = Left.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Left));
                            }
                        }
                        Down => {
                            if let Some((new_x, new_y)) = Right.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Right));
                            }
                        }
                        Left => {
                            if let Some((new_x, new_y)) = Up.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Up));
                            }
                        }
                        Right => {
                            if let Some((new_x, new_y)) = Down.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Down));
                            }
                        }
                    },
                    '-' => match direction {
                        Up | Down => {
                            if let Some((new_x, new_y)) = Left.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Left));
                            }
                            if let Some((new_x, new_y)) = Right.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Right));
                            }
                        }
                        Left | Right => {
                            if let Some((new_x, new_y)) =
                                direction.move_in_direction(&grid, &(x, y))
                            {
                                beams.push((new_x, new_y, direction));
                            }
                        }
                    },
                    '|' => match direction {
                        Up | Down => {
                            if let Some((new_x, new_y)) =
                                direction.move_in_direction(&grid, &(x, y))
                            {
                                beams.push((new_x, new_y, direction));
                            }
                        }
                        Left | Right => {
                            if let Some((new_x, new_y)) = Up.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Up));
                            }
                            if let Some((new_x, new_y)) = Down.move_in_direction(&grid, &(x, y)) {
                                beams.push((new_x, new_y, Down));
                            }
                        }
                    },
                    _ => unreachable!(),
                }
            }
        }
        let energized: HashSet<(usize, usize)> = walked.iter().map(|(x, y, _)| (*x, *y)).collect();

        max_energy = max(energized.len(), max_energy);
    }
    max_energy.to_string()
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
        let input: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(part_2(input), "51");
    }
}
