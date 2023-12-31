use std::collections::BTreeMap;
use num::integer::lcm;

enum Direction {
    Left,
    Right,
}

fn parse(input: &str) -> (Vec<Direction>, BTreeMap<&str, (&str, &str)>){
    let (directions, input) = input.split_once("\n\n").unwrap();
    let directions = directions.chars().map(|c| {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }).collect();

    let map: BTreeMap<&str, (&str, &str)> = input.lines().map(|l| {
        let (position, destinations) = l.split_once(" = ").unwrap();
        let (left, right) = destinations.split_once(", ").unwrap();

        (position, (&left[1..], &right[..right.len() - 1]))
    }).collect();
    (directions, map)
}

fn part_2(input: &str) -> String {
    let (directions, map) = parse(input);
    let start_positions: Vec<&str> = map.keys().filter(|k| k.chars().nth(2) == Some('A')).copied().collect();
    let mut end_nodes: Vec<usize> = Vec::with_capacity(6);
    for start_position in start_positions.iter() {
        let mut position = start_position;
        let mut step_count: usize = 0;

        // After investigating the data a bit a turns out there are loops with
        // exacly one endpoin for each start position. These loops are not
        // looping to the starting elements but they have the same lenght
        // anyway. So we can take the position of the rist end point we can
        // find for eatch starting position.
        for direction in directions.iter().cycle() {
            let (l, r) = map.get(position).unwrap();
            match direction {
                Direction::Left => position = l,
                Direction::Right => position = r,
            }
            step_count += 1;

            if position.as_bytes()[2] == b'Z' {
                end_nodes.push(step_count);
                break;
            }
        }
    }

    end_nodes.iter().fold(1usize, |acc, distance| lcm(acc, *distance)).to_string()
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
        let input: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        assert_eq!(part_2(input), "6");
    }
}
