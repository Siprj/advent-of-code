use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

const DIRECTIONS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

fn part_2(input: &str) -> String {
    let table = parse(input);
    let width = table[0].len() as i32;
    let height = table.len() as i32;
    let word = "MAS";
    let first_char = word.chars().next().unwrap();
    let mut a_map: HashMap<(i32, i32), u32> = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            if table[y as usize][x as usize] == first_char {
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
                    a_map
                        .entry((x + dx, y + dy))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
            }
        }
    }
    let count = a_map.values().filter(|&&a| a == 2).count();
    count.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);
    println!("Part 2: {}", result);
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
        assert_eq!(part_2(input), "9");
    }
}
