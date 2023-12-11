
fn parse(input: &str, multiplier: usize) -> Vec<(usize,usize)> {
    let input:Vec<Vec<char>> = input.lines().map(|l|l.chars().collect()).collect();
    let mut y = 0;
    let mut output: Vec<(usize, usize)> = vec![];
    let empty_comulns: Vec<usize> = (0..input[0].len()).filter(|x| {
        (0..input.len()).all(|y| input[y][*x] != '#')
    }).collect();

    for l in input.iter() {
        let mut empty = true;
        for (x, c) in l.iter().enumerate() {
            if c == &'#' {
                let x = x + empty_comulns.iter().filter(|c| x > **c).count() * (multiplier - 1);
                output.push((x, y));
                empty = false;
            }
        }
        if empty {
            y += multiplier;
        } else {
            y += 1;
        }
    }
    output
}

fn part_2(input: &str, multiplier: usize) -> String {
    let pairs = parse(input, multiplier);
    let mut sum: usize = 0;
    for (index, (start_x, start_y)) in pairs[..pairs.len() - 1].iter().enumerate() {
        for (end_x, end_y) in &pairs[index + 1..] {
            let dist = end_x.abs_diff(*start_x) + end_y.abs_diff(*start_y);
            sum += dist;
        }
    }
    sum.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input, 1_000_000);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part_2(input, 2), "374");
        assert_eq!(part_2(input, 10), "1030");
        assert_eq!(part_2(input, 100), "8410");
    }
}
