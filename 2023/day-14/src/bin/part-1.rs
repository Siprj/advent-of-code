fn parse(input: &str) -> Vec<Vec<char>> {
    let platform: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // Transpose for easier handling
    (0..platform[0].len())
        .map(|x| (0..platform.len()).map(|y| platform[y][x]).collect())
        .collect()
}

fn part_1(input: &str) -> String {
    let mut platform = parse(input);
    // The platform is transposed so we iterate over collumns.
    for column in platform.iter_mut() {
        loop {
            let mut moved = false;
            for i in 0..column.len() - 1 {
                if column[i] == '.' {
                    if column[i + 1] == 'O' {
                        column[i] = column[i + 1];
                        column[i + 1] = '.';
                        moved = true;
                    }
                }
            }
            if !moved {
                break;
            }
        }
    }

    let mut sum = 0;
    for column in platform {
        for (i, elem) in column.iter().enumerate() {
            if elem == &'O' {
                sum += column.len() - i;
            }
        }
    }
    sum.to_string()
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
        let input: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part_1(input), "136");
    }
}
