#[derive(Debug)]
struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(' ').unwrap();
            let (px, py) = p.split_once(',').unwrap();
            let p = (px[2..].parse::<i64>().unwrap(), py.parse::<i64>().unwrap());
            let (vx, vy) = v.split_once(',').unwrap();
            let v = (vx[2..].parse::<i64>().unwrap(), vy.parse::<i64>().unwrap());
            Robot { p, v }
        })
        .collect()
}

const SECONDS: i64 = 100;

//fn print_map(robots: &[Robot], width: i64, height: i64) {
//    let mut map: Vec<Vec<i64>> = (0..height).map(|_|(0..width).map(|_| 0).collect()).collect();
//    for r in robots.iter() {
//        map[r.p.1 as usize][r.p.0 as usize] += 1;
//    }
//    for l in map.iter() {
//        for n in l.iter() {
//            if *n > 0 {
//                print!("{n}");
//            } else {
//                print!(".");
//            }
//        }
//        println!();
//    }
//}

fn part_1(input: &str, width: i64, height: i64) -> String {
    let mut robots = parse(input);

    for robot in robots.iter_mut() {
        robot.p = (
            (robot.p.0 + (robot.v.0 * SECONDS)).rem_euclid(width),
            (robot.p.1 + (robot.v.1 * SECONDS)).rem_euclid(height),
        );
    }
    //print_map(&robots, width, height);
    let half_width = width / 2;
    let half_height = height / 2;

    let (ul, ur, dl, dr) = robots
        .iter()
        .fold((0i64, 0i64, 0i64, 0i64), |(ul, ur, dl, dr), r| {
            if r.p.0 < half_width {
                if r.p.1 < half_height {
                    (ul + 1, ur, dl, dr)
                } else if r.p.1 == half_height {
                    (ul, ur, dl, dr)
                } else {
                    (ul, ur, dl + 1, dr)
                }
            } else if r.p.0 == half_width {
                (ul, ur, dl, dr)
            } else if r.p.1 < half_height {
                (ul, ur + 1, dl, dr)
            } else if r.p.1 == half_height {
                (ul, ur, dl, dr)
            } else {
                (ul, ur, dl, dr + 1)
            }
        });

    (ul * ur * dl * dr).to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input, 101, 103);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(part_1(input, 11, 7), "12");
    }
}
