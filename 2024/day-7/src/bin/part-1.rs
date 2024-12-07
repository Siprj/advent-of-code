fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let (test, values) = l.split_once(": ").unwrap();
            let values: Vec<u64> = values
                .split_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect();
            (test.parse::<u64>().unwrap(), values)
        })
        .collect()
}

fn part_1(input: &str) -> String {
    let lines = parse(input);
    let mut sum = 0;
    for l in lines.iter() {
        let test = l.0;
        let numbers = &l.1;
        let mut posiblities: Vec<u64> = vec![0];
        for n in numbers.iter() {
            let mut tmp = vec![];
            for p in posiblities {
                let mul = p * n;
                if mul <= test && mul != 0 {
                    tmp.push(mul);
                }
                let add = p + n;
                if add <= test && add != 0 {
                    tmp.push(add);
                }
            }
            posiblities = tmp;
        }
        for p in posiblities.iter() {
            if test == *p {
                sum += test;
                break;
            }
        }
    }

    sum.to_string()
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
        let input: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part_1(input), "3749");
    }
}
