use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect()).collect()
}

fn part_2(input: &str) -> String {
    let sequences = parse(input);
    let mut sum:i32 = 0;
    for sequence in sequences {
        let mut sub_sequences: Vec<Vec<i32>> = Vec::with_capacity(sequence.len());
        sub_sequences.push(sequence);
        loop {
            let sequence = sub_sequences.last().unwrap();
            let new_sequence: Vec<i32> = sequence.iter().tuple_windows().map(|(a,b)| b - a).collect();
            if new_sequence.iter().all(|n| n == &0) {
                sub_sequences.push(new_sequence);
                break;
            } else {
                sub_sequences.push(new_sequence);
            }
        }
        let mut addend = 0;
        for sub_sequence in sub_sequences.iter().rev() {
            let first = sub_sequence.first().unwrap_or(&0);
            addend = first - addend;
        }
        sum += addend;
    }
    sum.to_string()
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
        let input: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part_2(input), "2");
    }
}
