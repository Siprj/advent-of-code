use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<Vec<char>> {
    let input = input.trim();
    input.lines().map(|bank| bank.chars().collect()).collect()
}

pub fn get_adjacent_rolls(x: isize, y: isize, grid: &[Vec<char>]) -> HashSet<(isize, isize)> {
    let spots: &[(isize, isize)] = &[
        (1, 1),
        (1, -1),
        (-1, -1),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];

    spots
        .iter()
        .filter_map(|(dx, dy)| {
            let x_ = dx + x;
            let y_ = dy + y;
            if x_ >= 0
                && x_ < grid[0].len() as isize
                && y_ >= 0
                && y_ < grid.len() as isize
                && grid[y_ as usize][x_ as usize] == '@'
            {
                Some((x_, y_))
            } else {
                None
            }
        })
        .collect()
}

pub fn get_accessable_rolls(grid: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut rolls = HashSet::new();
    let x_len = grid[0].len();
    let y_len = grid.len();
    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] == '@' && get_adjacent_rolls(x as isize, y as isize, grid).len() < 4 {
                rolls.insert((x, y));
            }
        }
    }
    rolls
}

pub fn empty_positions(grid: &mut [Vec<char>], positions: HashSet<(usize, usize)>) {
    for (x, y) in positions.iter() {
        grid[*y][*x] = '.';
    }
}

pub fn print_grid(grid: &[Vec<char>]) {
    for line in grid {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}
