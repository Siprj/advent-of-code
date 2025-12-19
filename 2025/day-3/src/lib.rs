pub fn parse(input: &str) -> Vec<Vec<char>> {
    let input = input.trim();
    input.lines().map(|bank| bank.chars().collect()).collect()
}

pub fn get_max(bank: &[char]) -> (usize, char) {
    let mut digit = '0';
    let mut index = 0;
    for (i, d) in bank.iter().enumerate() {
        if *d > digit {
            digit = *d;
            index = i;
        }
    }
    (index, digit)
}

pub fn get_joltage(bank: &[char], digits: usize) -> u64 {
    let mut index = 0;
    let mut res: Vec<char> = vec![];
    for restriction in (0..digits).rev() {
        let (i, d) = get_max(&bank[index..(bank.len() - restriction)]);
        index += i + 1;
        res.push(d);
    }
    let str: String = res.iter().collect();
    str.parse().unwrap()
}
