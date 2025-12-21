pub fn parse(input: &str) -> Vec<(isize, isize)> {
    let input = input.trim();
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}
