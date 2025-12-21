use day_9::parse;

fn part_2(input: &str) -> String {
    let boxes = parse(input);
    todo!()
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
        let input: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part_2(input), "25272");
    }
}
