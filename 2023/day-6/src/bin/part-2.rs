fn parse_input(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_once(":").unwrap().1;
    let time = times.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let time = time.parse::<u64>().unwrap();
    let distances = lines.next().unwrap().split_once(":").unwrap().1;
    let distance = distances.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let distance = distance.parse::<u64>().unwrap();
    (time, distance)
}

fn part_2(input: &str) -> String {
    let (time, distance) = parse_input(input);
    (1..time).map(|td| td * (time - td)).filter(|my_distance| my_distance > &distance).collect::<Vec<u64>>().len().to_string()
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
        let input: &str = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part_2(input), "71503");
    }
}
