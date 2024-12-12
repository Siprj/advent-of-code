use std::collections::HashSet;

type Map<T> = Vec<Vec<T>>;

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    contains: HashSet<(i32, i32)>,
}

fn parse(input: &str) -> Map<char> {
    input.trim().lines().map(|l| l.chars().collect()).collect()
}

fn next(
    fill: &Map<usize>,
    next_point: (i32, i32),
    size: &(i32, i32),
    index: usize,
    stack: &mut Vec<(i32, i32)>,
    region: &mut Region,
) {
    if is_valid_point(&next_point, size) {
        if fill[next_point.0 as usize][next_point.1 as usize] != index {
            stack.push(next_point);
        }
    } else {
        region.perimeter += 1;
    }
}

fn flood_fill(
    map: &Map<char>,
    fill: &mut Map<usize>,
    c: char,
    index: usize,
    region: &mut Region,
    start: (i32, i32),
    size: (i32, i32),
) {
    let mut stack = vec![start];

    while let Some(current) = stack.pop() {
        if fill[current.0 as usize][current.1 as usize] == index {
            continue;
        }
        if map[current.0 as usize][current.1 as usize] == c {
            region.area += 1;
            region.contains.insert(current);
            fill[current.0 as usize][current.1 as usize] = index;

            let next_point = (current.0 + 1, current.1);
            next(fill, next_point, &size, index, &mut stack, region);
            let next_point = (current.0, current.1 + 1);
            next(fill, next_point, &size, index, &mut stack, region);
            let next_point = (current.0, current.1 - 1);
            next(fill, next_point, &size, index, &mut stack, region);
            let next_point = (current.0 - 1, current.1);
            next(fill, next_point, &size, index, &mut stack, region);
        } else {
            region.perimeter += 1;
        }
    }
}

const DIR: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

fn is_valid_point(point: &(i32, i32), size: &(i32, i32)) -> bool {
    point.0 >= 0 && point.0 < size.0 && point.1 >= 0 && point.1 < size.1
}

fn is_corner(
    contains: &HashSet<(i32, i32)>,
    pos: &(i32, i32),
    dir: usize,
    size: &(i32, i32),
) -> bool {
    let next = (pos.0 + DIR[dir].0, pos.1 + DIR[dir].1);
    let test_dir = (dir + 1) % 4;
    let up_pos = (pos.0 + DIR[test_dir].0, pos.1 + DIR[test_dir].1);
    let diag_pos = (next.0 + DIR[test_dir].0, next.1 + DIR[test_dir].1);

    let next_valid = is_valid_point(&next, size) && contains.contains(&next);
    let up_valid = is_valid_point(&up_pos, size) && contains.contains(&up_pos);
    let diag_valid = is_valid_point(&diag_pos, size) && contains.contains(&diag_pos);

    if !next_valid && !up_valid {
        true
    } else {
        !up_valid && diag_valid && next_valid
    }
}

fn part_2(input: &str) -> String {
    let map = parse(input);
    let height = map.len();
    let width = map[0].len();
    let mut fill_map: Map<usize> = (0..height)
        .map(|_| (0..width).map(|_| usize::MAX).collect())
        .collect();
    let mut regions: Vec<Region> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if fill_map[y][x] == usize::MAX {
                regions.push(Region {
                    area: 0,
                    perimeter: 0,
                    contains: HashSet::new(),
                });
                flood_fill(
                    &map,
                    &mut fill_map,
                    map[y][x],
                    regions.len() - 1,
                    regions.last_mut().unwrap(),
                    (y as i32, x as i32),
                    (height as i32, width as i32),
                );
            }
        }
    }

    for region in regions.iter_mut() {
        let mut corners = 0;
        for d in 0..4 {
            for p in region.contains.iter() {
                if is_corner(&region.contains, p, d, &(height as i32, width as i32)) {
                    corners += 1;
                }
            }
        }
        region.perimeter = corners;
    }

    regions
        .iter()
        .map(|r| r.perimeter * r.area)
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let len = input.len();
    let result = part_2(&input[..len - 1]);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part_2(input), "80");
    }

    #[test]
    fn it_works2() {
        let input: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part_2(input), "436");
    }

    #[test]
    fn it_works3() {
        let input: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part_2(input), "1206");
    }
}
