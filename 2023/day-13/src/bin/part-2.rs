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

fn hamming_horizontal_distance(line1: &[char], line2: &[char]) -> usize {
    let mut distance = 0;
    for (c1, c2) in line1.iter().zip(line2.iter()) {
        if c1 != c2 {
            distance += 1;
        }
    }
    distance
}

fn hamming_vertical_distance(section: &[Vec<char>], c1: usize, c2: usize) -> usize {
    (0..section.len()).fold(0, |acc, i| {
        if section[i][c1] != section[i][c2] {
            acc + 1
        } else {
            acc
        }
    })
}

fn find_vertical(section: &[Vec<char>]) -> Option<usize> {
    (0..section[0].len() - 1)
        .find(|&i| {
            let mut used = false;
            (0..=i).rev().zip(i + 1..section[0].len()).all(|(i, ii)| {
                let distance = hamming_vertical_distance(section, i, ii);
                if distance == 0 {
                    true
                } else if distance == 1 && !used {
                    used = true;
                    true
                } else {
                    false
                }
            }) && used
        })
        .map(|i| i + 1)
}

fn find_horizontal(section: &Vec<Vec<char>>) -> Option<usize> {
    (0..section.len() - 1)
        .find(|&i| {
            let mut used = false;
            (0..=i)
                .rev()
                .zip(i + 1..section.len())
                .all(|(i, ii)| {
                let distance = hamming_horizontal_distance(&section[i], &section[ii]);
                if distance == 0 {
                    true
                } else if distance == 1 && !used {
                    used = true;
                    true
                } else {
                    false
                }
                }) && used
        })
        .map(|i| i + 1)
}

fn part_2(input: &str) -> String {
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
    let result = part_2(input);
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
        assert_eq!(part_2(input), "400");
    }
}
