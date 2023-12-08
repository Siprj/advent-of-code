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

fn part_1(input: &str) -> String {
    let (directions, map) = parse(input);
    let direction_loop = directions.iter().cycle();
    let mut step_count: u32 = 0;
    let mut position: &str = "AAA";
    for direction in direction_loop {
        let (l, r) = map.get(position).unwrap();
        match direction {
            Direction::Left => position = l,
            Direction::Right => position = r,
        }
        step_count += 1;
        if position == "ZZZ" {
            break;
        }
    }

    step_count.to_string()
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
        let input: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(part_1(input), "2");
    }

    #[test]
    fn it_works_2() {
        let input: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(part_1(input), "6");
    }
}
