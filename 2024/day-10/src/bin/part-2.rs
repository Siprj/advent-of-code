fn parse(input: &str) -> Map {
    input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

const DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

type Map = Vec<Vec<u32>>;
type Res = Vec<((i32, i32), (i32, i32))>;

fn step(map: &Map, pos: &(i32, i32), start: &(i32, i32), size: &(i32, i32), res: &mut Res) {
    let n = map[pos.0 as usize][pos.1 as usize];
    if n == 9 {
        res.push((*start, *pos));
    }
    for (dy, dx) in DIR.iter() {
        let y = pos.0 + *dy;
        let x = pos.1 + *dx;
        if y >= 0 && y < size.0 && x >= 0 && x < size.1 {
            let n2 = map[y as usize][x as usize];
            if n + 1 == n2 {
                step(map, &(y, x), start, size, res);
            }
        }
    }
}

fn part_2(input: &str) -> String {
    let map = parse(input);
    let size = (map.len() as i32, map[0].len() as i32);
    let starts: Vec<(i32, i32)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter().enumerate().filter_map(move |(x, c)| {
                if *c == 0 {
                    Some((y as i32, x as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut sum = 0;

    for s in starts.iter() {
        let mut res = Vec::new();
        step(&map, s, s, &size, &mut res);
        sum += res.len();
    }
    sum.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let len = input.len();
    let result = part_2(&input[..len - 1]);
    println!("Part 2: {}", result);
}
