pub fn parse(input: &str) -> (Vec<Vec<char>>, usize) {
    let input = input.trim();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = grid[0].iter().position(|v| *v == 'S').unwrap();
    (grid, start)
}
