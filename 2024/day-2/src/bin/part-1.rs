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

fn part_1(input: &str) -> String {
    let lines = parse(input);
    let mut valid_lines = 0;
    'outer: for line in lines.into_iter() {
        let asc = line[0] < line[1];
        for n in line.as_slice().windows(2) {
            let n0 = n[0];
            let n1 = n[1];
            let abs_diff = n0.abs_diff(n1);
            if (n0 < n1) != asc || !(1..=3).contains(&abs_diff) {
                continue 'outer;
            }
        }
        valid_lines += 1;
    }
    valid_lines.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);
    println!("Part 1: {}", result);
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
        assert_eq!(part_1(input), "2");
    }
}
