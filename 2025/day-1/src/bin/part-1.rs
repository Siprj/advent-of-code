enum Direction {
    Right,
    Left,
}
fn parse(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(|l| {
            let (direction, number) = l.split_at(1);
            let direction = match direction.chars().next().unwrap() {
                'R' => Direction::Right,
                'L' => Direction::Left,
                d => panic!("Unknonw direction: {}", d),
            };

            (direction, number.parse::<i32>().unwrap())
        })
        .collect()
}

fn part_1(input: &str) -> String {
    let moves = parse(input);
    let mut dial: i32 = 50;
    let mut zero_count = 0;
    for (direction, number) in moves.iter() {
        match direction {
            Direction::Right => {
                dial = (dial + number) % 100;
            }
            Direction::Left => dial = (dial - number).rem_euclid(100),
        }
        if dial == 0 {
            zero_count += 1;
        }
    }
    zero_count.to_string()
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
        let input: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part_1(input), "3");
    }
}
