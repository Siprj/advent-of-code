fn part_1(input: &str) -> String {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let (l, r) = line.split_once(':').unwrap();
        let id = l.split_whitespace().nth(1).unwrap();
        let mut max_green = 0;
        let mut max_blue = 0;
        let mut max_red = 0;
        for hand in r.split(';') {
            for cube in hand.split(", ").collect::<Vec<&str>>() {
                let (count, color) = cube.trim().split_once(' ').unwrap();
                let count: u32 = count.parse().unwrap();
                match color {
                    "blue" => max_blue = max_blue.max(count),
                    "green" => max_green = max_green.max(count),
                    "red" => max_red = max_red.max(count),
                    _ => {}
                }
            }
        }
        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            sum += id.parse::<u32>().unwrap();
        }
    }
    sum.to_string()
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
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_1(input), "8");
    }
}
