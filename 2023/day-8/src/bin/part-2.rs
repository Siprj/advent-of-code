use std::collections::BTreeMap;

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
    let direction_loop = directions.iter().cycle();
    let mut step_count: u32 = 0;
    let start_positions: Vec<&str> = map.keys().filter(|k| k.chars().nth(2) == Some('A')).copied().collect();
    let mut positions: Vec<&str> = start_positions.clone();
    println!("start_positions.len(): {}", positions.len());
    println!("start_positions: {:?}", start_positions);
    for direction in direction_loop {
        for position in positions.iter_mut() {
            let (l, r) = map.get(position).unwrap();
            match direction {
                Direction::Left => *position = *l,
                Direction::Right => *position = *r,
            }
        }
        step_count += 1;
        if positions.iter().all(|position| position.chars().nth(2) == Some('Z')) {
            break;
        }
    }

    step_count.to_string()
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
