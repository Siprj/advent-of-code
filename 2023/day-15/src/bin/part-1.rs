fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .split(',')
        .map(|l| {
            l.as_bytes()
                .iter()
                .copied()
                .filter(|c| *c != b'\n')
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn part_1(input: &str) -> String {
    let parts = parse(input);
    parts
        .iter()
        .map(|part| {
            part.iter()
                .fold(0u64, |acc, c| ((acc + *c as u64) * 17) % 256)
        })
        .sum::<u64>()
        .to_string()
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
        let input: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part_1(input), "1320");
    }
}
