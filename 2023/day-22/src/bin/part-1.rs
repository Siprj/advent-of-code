#[derive(Debug, Clone)]
struct Brick {
    x: u32,
    y: u32,
    z: u32,
    x2: u32,
    y2: u32,
    z2: u32,
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once('~').unwrap();
            let (x, rest) = left.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();
            let (x2, rest) = right.split_once(',').unwrap();
            let (y2, z2) = rest.split_once(',').unwrap();
            Brick {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
                x2: x2.parse().unwrap(),
                y2: y2.parse().unwrap(),
                z2: z2.parse().unwrap(),
            }
        })
        .collect()
}

fn get_max(bricks: &Vec<Brick>) -> (u32, u32, u32) {
    let x_max = bricks.iter().map(|b| b.x2).max().unwrap();
    let y_max = bricks.iter().map(|b| b.y2).max().unwrap();
    let z_max = bricks.iter().map(|b| b.z2).max().unwrap();
    (x_max, y_max, z_max)
}

fn part_1(input: &str) -> String {
    let bricks = parse(input);
    dbg!(&bricks);
    let (x_max, y_max, z_max) = get_max(&bricks);
    dbg!(&max);

    for

    todo!()
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
        let input: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(part_1(input), "5");
    }
}
