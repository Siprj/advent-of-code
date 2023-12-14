use std::collections::{HashSet, HashMap};

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn move_north(platform: &mut Vec<Vec<char>>) {
    for x in 0..platform[0].len() {
        loop {
            let mut moved = false;
            for y in 0..platform.len() - 1 {
                if platform[y][x] == '.' {
                    if platform[y + 1][x] == 'O' {
                        platform[y][x] = platform[y + 1][x];
                        platform[y + 1][x] = '.';
                        moved = true;
                    }
                }
            }
            if !moved {
                break;
            }
        }
    }
}

fn move_south(platform: &mut Vec<Vec<char>>) {
    for x in 0..platform[0].len() {
        loop {
            let mut moved = false;
            for y in (1..platform.len()).rev() {
                if platform[y][x] == '.' {
                    if platform[y - 1][x] == 'O' {
                        platform[y][x] = platform[y - 1][x];
                        platform[y - 1][x] = '.';
                        moved = true;
                    }
                }
            }
            if !moved {
                break;
            }
        }
    }
}

fn move_west(platform: &mut Vec<Vec<char>>) {
    for row in platform.iter_mut() {
        loop {
            let mut moved = false;
            for x in 0..row.len() - 1{
                if row[x] == '.' {
                    if row[x + 1] == 'O' {
                        row[x] = row[x + 1];
                        row[x + 1] = '.';
                        moved = true;
                    }
                }
            }
            if !moved {
                break;
            }
        }
    }
}

fn move_east(platform: &mut Vec<Vec<char>>) {
    for row in platform.iter_mut() {
        loop {
            let mut moved = false;
            for x in (1..row.len()).rev() {
                if row[x] == '.' {
                    if row[x - 1] == 'O' {
                        row[x] = row[x - 1];
                        row[x - 1] = '.';
                        moved = true;
                    }
                }
            }
            if !moved {
                break;
            }
        }
    }
}

fn part_2(input: &str) -> String {
    let mut platform = parse(input);
    // The platform is transposed so we iterate over collumns.
    let mut iterations: Vec<(usize, usize)> = vec![];
    let mut index_map: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    for i in 0..1000 {
        move_north(&mut platform);
        move_west(&mut platform);
        move_south(&mut platform);
        move_east(&mut platform);
        let mut sum = 0;
        for x in 0..platform[0].len() {
            for y in 0..platform.len() {
                if platform[y][x] == 'O' {
                    sum += platform.len() - y;
                }
            }
        }
        println!("iteration: {}, stress: {}",i, sum);
        iterations.push((i, sum));
        if index_map.contains_key(&platform){
            let kwa = index_map.get(&platform).unwrap();
            println!("index: {}, kwa: {}", i, kwa);
            break;
        } else {
            index_map.insert(platform.clone(), i);
        }
    }
    todo!();
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
        assert_eq!(part_2(input), "136");
    }
}
