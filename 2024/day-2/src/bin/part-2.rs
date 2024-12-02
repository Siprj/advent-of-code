fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn check_line<'a, I>(mut iter: I) -> bool
where
    I: Iterator<Item = &'a u32>,
{
    let mut n0 = *iter.next().unwrap();
    let n1: u32 = *iter.next().unwrap();
    let asc: bool = n0 < n1;
    if !check_tuple(n0, n1, asc) {
        return false;
    }

    n0 = n1;

    for n1 in iter {
        if !check_tuple(n0, *n1, asc) {
            return false;
        }
        n0 = *n1;
    }
    true
}

fn check_tuple(n0: u32, n1: u32, asc: bool) -> bool {
    let abs_diff = n0.abs_diff(n1);
    ((n0 < n1) == asc) && (1..=3).contains(&abs_diff)
}

fn part_2(input: &str) -> String {
    let lines = parse(input);
    let mut valid_lines = 0;
    for l in lines.iter() {
        if check_line(l.iter()) {
            valid_lines += 1;
        } else {
            for i in 0..l.len() {
                let iter = l
                    .iter()
                    .enumerate()
                    .filter_map(|(x, v)| if x != i { Some(v) } else { None });
                if check_line(iter) {
                    valid_lines += 1;
                    break;
                }
            }
        }
    }
    valid_lines.to_string()
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
        let input: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part_2(input), "4");
    }
}
