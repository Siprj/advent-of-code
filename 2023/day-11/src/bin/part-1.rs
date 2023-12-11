
fn parse(input: &str) -> Vec<(usize,usize)> {
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
                let x = x + empty_comulns.iter().filter(|c| x > **c).count();
                output.push((x, y));
                empty = false;
            }
        }
        if empty {
            y += 2;
        } else {
            y += 1;
        }
    }
    output
}

fn part_1(input: &str) -> String {
    let pairs = parse(input);
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
    let result = part_1(input);
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
        assert_eq!(part_1(input), "374");
    }
}
