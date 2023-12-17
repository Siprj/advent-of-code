use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::usize;

const DIRECTION_LIMIT: i32 = 9;
const DIRECTION_LOW_LIMIT: i32 = 3;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
struct MoveState {
    direction: Direction,
    position: (isize, isize),
    steps: i32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct State {
    cost: u32,
    move_state: MoveState,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn direction_to_move(direction: &Direction) -> (isize, isize) {
    match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

fn next_positions(move_state: &MoveState, size: &(isize, isize)) -> Vec<MoveState> {
    let mut moves = if move_state.steps >= DIRECTION_LOW_LIMIT {
        match move_state.direction {
            Direction::Up | Direction::Down => vec![(Direction::Left, 0), (Direction::Right, 0)],
            Direction::Left | Direction::Right => vec![(Direction::Up, 0), (Direction::Down, 0)],
        }
    } else {
        vec![]
    };

    if move_state.steps < DIRECTION_LIMIT {
        moves.push((move_state.direction, move_state.steps + 1));
    }

    moves
        .iter()
        .map(|(dir, steps)| {
            let m = direction_to_move(dir);
            let new_pos = (move_state.position.0 + m.0, move_state.position.1 + m.1);
            MoveState {
                direction: *dir,
                position: new_pos,
                steps: *steps,
            }
        })
        .filter(|moved| {
            moved.position.0 >= 0
                && moved.position.0 < size.0
                && moved.position.1 >= 0
                && moved.position.1 < size.1
        })
        .collect()
}

fn get_value(map: &[Vec<u32>], pos: &(isize, isize)) -> u32 {
    map[pos.1 as usize][pos.0 as usize]
}

fn shortest_path(map: &Vec<Vec<u32>>, goal: (isize, isize)) -> u32 {
    let mut visited: HashSet<MoveState> = HashSet::new();

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let size = (map[0].len() as isize, map.len() as isize);

    heap.push(State {
        cost: 0,
        move_state: MoveState {
            direction: Direction::Right,
            position: (0, 0),
            steps: -1,
        },
    });
    heap.push(State {
        cost: 0,
        move_state: MoveState {
            direction: Direction::Down,
            position: (0, 0),
            steps: -1,
        },
    });

    while let Some(best) = heap.pop() {
        if !visited.insert(best.move_state) {
            continue;
        }

        if best.move_state.position == goal && best.move_state.steps >= DIRECTION_LOW_LIMIT {
            return best.cost;
        }

        for next_position in next_positions(&best.move_state, &size) {
            if !visited.contains(&next_position) {
                heap.push(State {
                    cost: best.cost + get_value(map, &next_position.position),
                    move_state: next_position,
                });
            }
        }
    }
    unreachable!()
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_2(input: &str) -> String {
    let map = parse(input);
    shortest_path(
        &map,
        ((map[0].len() - 1) as isize, (map.len() - 1) as isize),
    )
    .to_string()
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
        let input: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(part_2(input), "94");
    }

    #[test]
    fn it_works_2() {
        let input: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(part_2(input), "71");
    }
}
