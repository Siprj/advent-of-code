use std::collections::{HashMap, VecDeque};

fn parse(input: &str) -> (Vec<Vec<char>>, (isize, isize)) {
    let mut map: Vec<Vec<char>> = input.lines().map(|line|line.chars().collect()).collect();
    let start = map.iter().enumerate().find_map(|(y, line)| line.iter().position(|c| c == &'S').map(|x| (x,y))).unwrap();
    map[start.1][start.0] = '.';
    (map, (start.0 as isize, start.1 as isize))
}

const NEXT: [(isize, isize);4] = [(-1, 0), (1,0), (0,1), (0,-1)];

fn valid_neighbor_positions(map: &Vec<Vec<char>>, position: &(isize, isize)) -> Vec<(isize, isize)>{
    let mut out = vec![];
    for next in NEXT {
        let next_x = next.0 + position.0;
        let next_y = next.1 + position.1;
        if map[next_y.rem_euclid(map.len() as isize) as usize][next_x.rem_euclid(map[0].len() as isize) as usize] != '#' {
            out.push((next_x, next_y));
        }
    }
    out
}

fn part_2(input: &str, number_of_steps: u64) -> String {
    let (map, start) = parse(input);
    let mut stack: VecDeque<(isize, isize, u64)> = VecDeque::with_capacity(1_000_000_000);
    stack.push_back((start.0, start.1, 0));
    let mut mem: HashMap<(isize, isize, u64), (usize, bool)> = HashMap::with_capacity(1_000_000_000);
//    dbg!(&start);
    let mut iter: usize = 0;

    while let Some((x, y, steps)) = stack.pop_back() {
        if iter % 1000  == 0{
            println!("iteration: {}, steps: {}, stack.len(): {}, mem.len(): {}", iter, steps, stack.len(), mem.len());
        }
        iter += 1;
//        dbg!((x,y,steps));
//        dbg!(valid_neighbor_positions(&map, &(x,y)));
        if steps == number_of_steps {
//            println!("before last");
            mem.insert((x, y, steps), (1, false));
        } else {
            let mut local_sum = 0;
            let mut next_evaluations = vec![];
            for next_pos in valid_neighbor_positions(&map, &(x,y)) {
                if let Some(next_sum) = mem.get(&(next_pos.0, next_pos.1, steps + 1)) {
                    if !next_sum.1 {
                        local_sum += next_sum.0;
                    }
                } else {
                    next_evaluations.push((next_pos.0, next_pos.1, steps + 1));
                }
            }
            if next_evaluations.is_empty() {
                if steps == 0 {
                    return local_sum.to_string()
                }
                for next_pos in valid_neighbor_positions(&map, &(x,y)) {
                     mem.get_mut(&(next_pos.0, next_pos.1, steps + 1)).unwrap().1 = true;
                }
//                dbg!(local_sum);
                mem.insert((x,y, steps), (local_sum, false));
            } else {
                stack.push_back((x,y, steps));
                for next_eval in next_evaluations {
                    stack.push_back(next_eval);
                }
            }
        }
    }
    unreachable!()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input, 26501365);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let input: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part_2(input, 6), "16");
    }

    #[test]
    fn it_works_2() {
        let input: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part_2(input, 10), "50");
    }
    #[test]
    fn it_works_3() {
        let input: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part_2(input, 50), "1594");
    }
}
