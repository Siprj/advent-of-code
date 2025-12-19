#[derive(Debug)]
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

fn part_2(input: &str) -> String {
    let moves = parse(input);
    let mut dial: i32 = 50;
    let mut zero_count = 0;
    for (direction, number) in moves.iter() {
        println!("before dial: {}", dial);
        println!("move: {:?} {}", direction, number);
        match direction {
            Direction::Right => {
                let acc = dial + number;
                zero_count += acc / 100;
                dial = acc % 100;
            }
            Direction::Left => {
                let acc = dial - number;
                if dial != 0 {
                    zero_count += (100 - dial + number) / 100;
                } else {
                    zero_count += (dial + number) / 100;
                }
                dial = acc.rem_euclid(100);
            }
        }
        println!("dial: {}", dial);
        println!("zero_count: {}", zero_count);
    }
    zero_count.to_string()
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
        assert_eq!(part_2(input), "6");
    }
}
