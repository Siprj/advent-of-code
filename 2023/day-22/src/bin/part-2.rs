use std::cmp::{max, min};

#[derive(Debug, Clone)]
struct Brick {
    x: u32,
    y: u32,
    z: i32,
    x2: u32,
    y2: u32,
    z2: i32,
}

impl Brick {
    fn move_down(&mut self) {
        self.z -= 1;
        self.z2 -= 1;
    }

    fn colision(&self, other: &Brick) -> bool {
        self.x <= other.x2
            && self.x2 >= other.x
            && self.y <= other.y2
            && self.y2 >= other.y
            && self.z <= other.z2
            && self.z2 >= other.z
    }

    fn nulify(&mut self) {
        self.x = 0;
        self.y = 0;
        self.z = 0;
        self.x2 = 0;
        self.y2 = 0;
        self.z2 = 0;
    }
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
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            let z = z.parse().unwrap();
            let x2 = x2.parse().unwrap();
            let y2 = y2.parse().unwrap();
            let z2 = z2.parse().unwrap();
            Brick {
                x: min(x, x2),
                y: min(y, y2),
                z: min(z, z2),
                x2: max(x, x2),
                y2: max(y, y2),
                z2: max(z, z2),
            }
        })
        .collect()
}

fn get_max(bricks: &[Brick]) -> (u32, u32, i32) {
    let x_max = bricks.iter().map(|b| b.x2).max().unwrap();
    let y_max = bricks.iter().map(|b| b.y2).max().unwrap();
    let z_max = bricks.iter().map(|b| b.z2).max().unwrap();
    (x_max, y_max, z_max)
}

fn part_2(input: &str) -> String {
    let mut bricks = parse(input);
    let mut moving = true;

    bricks.sort_by_key(|b| b.z);

    while moving {
        moving = false;
        for i in 0..bricks.len() {
            let mut b = bricks[i].clone();
            b.move_down();

            if !bricks[0..i].iter().any(|b2| b.colision(b2)) && b.z > 0 {
                bricks[i] = b;
                moving = true;
            }
        }
    }

    dbg!(get_max(&bricks));

    let mut sum: u32 = 0;
    for i in 0..bricks.len() {
        let mut copied_bricks = bricks.clone();
        copied_bricks[i].nulify();
        for i in 0..copied_bricks.len() {
            let mut b = copied_bricks[i].clone();
            b.move_down();

            if !copied_bricks[0..i].iter().any(|b2| b.colision(b2)) && b.z > 0 {
                copied_bricks[i] = b;
                sum += 1;
            }
        }
    }
    sum.to_string()
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
        let input: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(part_2(input), "7");
    }
}
