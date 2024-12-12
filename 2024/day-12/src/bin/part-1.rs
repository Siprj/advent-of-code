type Map<T> = Vec<Vec<T>>;

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
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
    if next_point.0 >= 0 && next_point.0 < size.0 && next_point.1 >= 0 && next_point.1 < size.1 {
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

fn part_1(input: &str) -> String {
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

    regions
        .iter()
        .map(|r| r.perimeter * r.area)
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let len = input.len();
    let result = part_1(&input[..len - 1]);
    println!("Part 1: {}", result);
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
        assert_eq!(part_1(input), "140");
    }

    #[test]
    fn it_works2() {
        let input: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part_1(input), "772");
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
        assert_eq!(part_1(input), "1930");
    }
}
