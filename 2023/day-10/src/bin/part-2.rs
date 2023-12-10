use std::collections::BTreeSet;

type Map = Vec<Vec<char>>;

fn parse(input: &str) -> ((i32, i32), Map) {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .enumerate()
                .find_map(|(x, c)| if c == &'S' { Some(x) } else { None }).map(|x| (x as i32, y as i32))
        })
        .unwrap();
    (start, map)
}

fn char_to_diffs(c: &char) -> [(i32, i32); 2] {
    match c {
        '-' => [( 1, 0 ), ( -1, 0)],
        '|' => [( 0, 1 ), ( 0, -1)],
        'L' => [( 1, 0 ), ( 0, -1)],
        'J' => [( -1, 0 ), ( 0,  -1)],
        '7' => [( -1, 0 ), ( 0, 1 )],
        'F' => [( 1, 0 ), ( 0, 1 )],
        'S' => [( 0, 0 ), ( 0, 0 )],
        '.' => [( 0, 0 ), ( 0, 0 )],
        _ => unreachable!(),
    }
}

fn add(x: &(i32, i32), y: &(i32, i32)) -> (i32, i32) {
    (x.0 + y.0, x.1 + y.1)
}

fn get_start_node(map: &Map, start: &(i32, i32)) -> (i32, i32) {
    for diff in [(-1, 0), (1, 0), (0, 1), (0, -1i32)].iter() {
        let next_pos = add(diff, start);
        if next_pos.1 >= 0 && next_pos.1 < map.len() as i32 && next_pos.0 >= 0 && next_pos.0 < map[0].len() as i32 {
            let char = map[next_pos.1 as usize][next_pos.0 as usize];
            for diff2 in char_to_diffs(&char).iter() {
                if &add(diff2, &next_pos) == start {
                    return next_pos;
                }
            }
        }
    }
    unreachable!()
}

fn follow_pipe(map: &Map, previous_pos: &(i32, i32), current_pos: &(i32, i32)) -> (i32, i32) {
    let char = map[current_pos.1 as usize][current_pos.0 as usize];
    let diffs = char_to_diffs(&char);
    let potential_next = add(&diffs[0], current_pos);
    if &potential_next == previous_pos {
        add(&diffs[1], current_pos)
    } else {
        potential_next
    }
}

fn part_2(input: &str) -> String {
    let (start, map) = parse(input);
    let mut current_pos = get_start_node(&map, &start);
    let mut previous_pos = start;
    let mut pipe_loop: BTreeSet<(i32, i32)> = BTreeSet::new();
    pipe_loop.insert(current_pos);
    while current_pos != start {
        let new_pos = follow_pipe(&map, &previous_pos, &current_pos);
        previous_pos = current_pos;
        current_pos = new_pos;
        pipe_loop.insert(current_pos);
    }

    let mut count: u32 = 0;
    for (y, l) in map.iter().enumerate() {
        for (x, _) in l.iter().enumerate() {
            if !pipe_loop.contains(&(x as i32, y as i32)) {
                let mut intersection_count = 0;
                let mut bla: Vec<(usize, i32)> = vec![];
                for x_diff in 0..=x {
                    let diag_y: i32 = (y as i32) - (x_diff as i32);
                    let diag_x: usize = ((x as i32) - (x_diff as i32)) as usize;
                    bla.push((diag_x, diag_y));
                    if diag_y >= 0 && map[diag_y as usize][diag_x] != '7' && map[diag_y as usize][diag_x] != 'L' && pipe_loop.contains(&(diag_x as i32, diag_y)) {
                        intersection_count += 1;
                    }
                }
                if intersection_count % 2 == 1 {
                    count += 1;
                }
            }
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
        let input: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(part_2(input), "4");
    }

    #[test]
    fn it_works_2() {
        let input: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(part_2(input), "4");
    }

    #[test]
    fn it_works_3() {
        let input: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(part_2(input), "8");
    }

    #[test]
    fn it_works_4() {
        let input: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part_2(input), "10");
    }

}
