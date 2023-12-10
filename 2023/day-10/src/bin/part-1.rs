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

fn part_1(input: &str) -> String {
    let (start, map) = parse(input);
    let mut count = 1;
    let mut current_pos = get_start_node(&map, &start);
    let mut previous_pos = start;
    while current_pos != start {
        count += 1;
        let new_pos = follow_pipe(&map, &previous_pos, &current_pos);
        previous_pos = current_pos;
        current_pos = new_pos;
    }
    (count/2).to_string()
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
        let input: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part_1(input), "4");
    }

    #[test]
    fn it_works_2() {
        let input: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(part_1(input), "8");
    }
}
