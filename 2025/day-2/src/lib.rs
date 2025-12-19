pub fn parse(input: &str) -> Vec<(u64, u64)> {
    let input = input.trim();
    input
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect()
}
