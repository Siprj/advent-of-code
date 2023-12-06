fn parse_input(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_once(':').unwrap().1;
    let time = times.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let time = time.parse::<u64>().unwrap();
    let distances = lines.next().unwrap().split_once(':').unwrap().1;
    let distance = distances.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let distance = distance.parse::<u64>().unwrap();
    (time, distance)
}

fn solve(t: i64, d:i64) -> i64 {
    // The equation we are trying solve is `(t - x) x = y` after rewriting it
    // we get `-x^2 + t = y` which is quadratic equation. If we move it using
    // the record on the y-axis such as => `-x^2 + t - d = y` we get number
    // over the record as positive numbers. So in the end we can approach it as
    // problem were we try to find the roots of the quadratic equation.

    let discriminant = (t*t) - 4 * d;
    if discriminant <= 0 {
        return 0;
    }
    // Coincidently the distance is between the two roots is
    // `discriminant.sqrt()`, so let's use that.
    let distance = (discriminant as f64).sqrt();
    // Also the parity is bit of mess......
    if t % 2 == 0 {
        2 * (distance / 2.0).ceil() as i64 - 1
    } else {
        2 * ((distance + 1.0) / 2.0).ceil() as i64 - 2
    }
}

fn part_2(input: &str) -> String {
    let (time, distance) = parse_input(input);

    solve(time as i64, distance as i64).to_string()
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
