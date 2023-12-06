use std::iter::zip;

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let mut lines = input.lines();
    let mut times = lines.next().unwrap().split_whitespace();
    times.next();
    let mut distances = lines.next().unwrap().split_whitespace();
    distances.next();
    zip(
        times.map(|s| s.parse::<u32>().expect("time should be a number")),
        distances.map(|s| s.parse::<u32>().expect("distance should be a number")),
    )
    .collect()
}
fn part_1(input: &str) -> String {
    let data = parse_input(input);
    let kwa = data.iter().map(|(t, d)| {
        (1..*t).map(|td| td * (t - td)).filter(|my_distance| my_distance > d).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    kwa.iter().map(|times| times.len()).product::<usize>().to_string()
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
        let input: &str = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part_1(input), "288");
    }
}
