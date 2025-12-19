pub fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| {
            let (direction, number) = l.split_at(1);
            match direction.chars().next().unwrap() {
                'R' => number.parse::<i32>().unwrap(),
                'L' => -number.parse::<i32>().unwrap(),
                d => panic!("Unknonw direction: {}", d),
            }
        })
        .collect()
}
