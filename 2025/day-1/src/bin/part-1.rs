use day_1::parse;

fn part_1(input: &str) -> String {
    let moves = parse(input);
    let mut dial: i32 = 50;
    let mut zero_count = 0;
    for number in moves.iter() {
        let acc = dial + number;
        if dial == 0 {
            zero_count += 1;
        }
        dial = acc.rem_euclid(100);
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
