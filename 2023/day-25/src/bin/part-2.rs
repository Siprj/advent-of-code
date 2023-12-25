use std::fs::read_to_string;

fn part_2(_input: &str) -> String {
    todo!()
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
        let input: &str = "";
        assert_eq!(part_2(input), "RESULT");
    }
}
