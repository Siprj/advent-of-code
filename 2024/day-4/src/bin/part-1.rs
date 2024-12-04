fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

fn part_1(input: &str) -> String {
    let table = parse(input);
    let width = table[0].len() as i32;
    let height = table.len() as i32;
    let mut count: u32 = 0;
    let word = "XMAS";
    for y in 0..height {
        for x in 0..width {
            if table[y as usize][x as usize] == 'X' {
                'direction: for (dx, dy) in DIRECTIONS {
                    for (i, c) in word.chars().enumerate().skip(1) {
                        let y = y + ((i as i32) * dy);
                        let x = x + ((i as i32) * dx);
                        if x < 0
                            || x >= width
                            || y < 0
                            || y >= height
                            || table[y as usize][x as usize] != c
                        {
                            continue 'direction;
                        }
                    }
                    count += 1;
                }
            }
        }
    }
    count.to_string()
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
        let input: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part_1(input), "18");
    }
}
