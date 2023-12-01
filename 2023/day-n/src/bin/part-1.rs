use std::fs::read_to_string;

fn part_1(_input: &str) -> String {
    todo!()
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
        let input: &str = "";
        assert_eq!(part_1(input), "RESULT");
    }
}
