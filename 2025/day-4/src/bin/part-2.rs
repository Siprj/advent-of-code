use day_4::{empty_positions, get_accessable_rolls, parse};

fn part_2(input: &str) -> String {
    let mut grid = parse(input);
    let mut count = 0;
    loop {
        let rolls = get_accessable_rolls(&grid);
        if rolls.is_empty() {
            break;
        }

        count += rolls.len();

        empty_positions(&mut grid, rolls);
    }
    count.to_string()
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
        let input: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part_2(input), "43");
    }
}
