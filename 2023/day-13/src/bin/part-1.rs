fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|section| {
            section
                .split_whitespace()
                .map(|l| l.chars().collect())
                .collect()
        })
        .collect()
}

fn equal_columns(section: &[Vec<char>], c1: usize, c2: usize) -> bool {
    (0..section.len()).all(|i| section[i][c1] == section[i][c2])
}

fn find_vertical(section: &[Vec<char>]) -> Option<usize> {
    (0..section[0].len() - 1)
        .find(|&i| {
            (0..=i)
                .rev()
                .zip(i + 1..section[0].len())
                .all(|(i, ii)| equal_columns(section, i, ii))
        })
        .map(|i| i + 1)
}

fn find_horizontal(section: &Vec<Vec<char>>) -> Option<usize> {
    (0..section.len() - 1)
        .find(|&i| {
            (0..=i)
                .rev()
                .zip(i + 1..section.len())
                .all(|(i, ii)| section[i] == section[ii])
        })
        .map(|i| i + 1)
}

fn part_1(input: &str) -> String {
    let sections = parse(input);
    let mut count = 0;
    for section in sections {
        if let Some(pivot) = find_horizontal(&section) {
            count += pivot * 100;
        }
        if let Some(pivot) = find_vertical(&section) {
            count += pivot;
        }
    }
    count.to_string()
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
        let input: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part_1(input), "405");
    }
}
