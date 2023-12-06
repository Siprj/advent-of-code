use std::iter::zip;

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

    data.iter().map(|(t, d)|solve(*t as i64, *d as i64)).product::<i64>().to_string()
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
