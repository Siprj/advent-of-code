use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

impl Coord {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Coord { x, y, z }
    }
    fn cross(&self, other: &Coord) -> Coord {
        Coord {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    fn abs(&self) -> f64 {
        let x = self.x as f64;
        let y = self.y as f64;
        let z = self.z as f64;
        (x*x + y*y + x*x).sqrt()
    }
}

#[derive(Debug, Clone)]
struct Hailstone {
    pos: Coord,
    vel: Coord,
}
impl Hailstone {
    fn new(pos: Vec<isize>, vel: Vec<isize>) -> Self {
        Hailstone {
            pos: Coord{ x: pos[0], y:pos[1], z:pos[2] },
            vel: Coord{ x: vel[0], y:vel[1], z:vel[2] },
        }
    }
}


fn get_intersection_2d(h1: &Hailstone, h2: &Hailstone) -> Option<(f64, f64)> {
    let determinant = (h1.vel.x * h2.vel.y - h1.vel.y * h2.vel.x) as f64;

    if determinant == 0.0 {
        return None;
    }
    let time: f64 = ((h2.pos.x - h1.pos.x) * h2.vel.y - (h2.pos.y - h1.pos.y) * h2.vel.x) as f64 / determinant;
    let time2: f64 = ((h1.pos.x - h2.pos.x) * h1.vel.y - (h1.pos.y - h2.pos.y) * h1.vel.x) as f64 / determinant;

    if time < 0.0 {
        return None;
    }
    if time * time2 > 0.0 {
        return None;
    }
    let x = h1.pos.x as f64 + h1.vel.x as f64* time;
    let y = h1.pos.y as f64 + h1.vel.y as f64 *time;

    Some((x, y))
}

fn part_a(input: &str, area: (f64, f64)) -> usize {
    let mut hailstones = Vec::new();
    for line in input.lines() {
        let (pos, vel) = line.split_once(" @ ").unwrap();
        let pos: Vec<isize> = pos.split(',').map(|p| p.trim().parse().unwrap()).collect();
        let vel: Vec<isize> = vel.split(',').map(|p| p.trim().parse().unwrap()).collect();
        hailstones.push(Hailstone::new(pos, vel))
    }
    let (lower, upper) = area;
    let mut collisions = 0;
    for combinations in hailstones.iter().combinations(2) {
        if let Some(intersection) = get_intersection_2d(combinations[0], combinations[1]){
            let (x, y) = intersection;

            if (x > lower && x < upper) && (y > lower && y < upper) {
                collisions +=1;
            }
        }
    }
    return collisions
}

fn find_intersection_3d(p1_0: Coord, v1: Coord, p2_0: Coord, v2: Coord) -> Option<Coord> {
    // cred https://math.stackexchange.com/questions/270767/find-intersection-of-two-3d-lines
    let g = Coord::new(p2_0.x - p1_0.x, p2_0.y - p1_0.y, p2_0.z - p1_0.z);
    let h = v2.cross(&g).abs();
    let k = v2.cross(&v1).abs();
    if h == 0.0 || k == 0.0{
        return None;
    }
    let intersection = Coord::new(
        p1_0.x + (v1.x as f64 *h/k).round() as isize,
        p1_0.y + (v1.y as f64 *h/k).round() as isize,
        p1_0.z + (v1.z as f64*h/k).round() as isize,
    );
    Some(intersection)
}

fn part_b(input: &str) -> Option<isize> {
    let mut hailstones = Vec::new();
    for line in input.lines() {
        let (pos, vel) = line.split_once(" @ ").unwrap();
        let pos: Vec<isize> = pos.split(',').map(|p| p.trim().parse().unwrap()).collect();
        let vel: Vec<isize> = vel.split(',').map(|p| p.trim().parse().unwrap()).collect();
        hailstones.push(Hailstone::new(pos, vel))
    }

    let max_vel = hailstones.iter().map(|x| x.vel.x.max(x.vel.y).max(x.vel.z)).max().unwrap();
    let min_vel = hailstones.iter().map(|x| x.vel.x.min(x.vel.y).min(x.vel.z)).min().unwrap();
    let max_vel = max_vel.max(min_vel.abs());

    let stones: Vec<Hailstone> = hailstones.iter().take(3).cloned().collect();

    let mut throw_pos = None;
    let max_vel = 300; // adjust as needed
    println!("Max vel: {}", max_vel);
    'outer: for vx in -max_vel..=max_vel {
        for vy in -max_vel..=max_vel {
            'vz: for vz in -max_vel..=max_vel {
                //println!("test V: {:?},{},{}", vx, vy, vz);
                let mut rel_hail = stones.clone();
                let mut intersect = None;
                for hail in rel_hail.iter_mut() {
                    hail.vel.x = hail.vel.x + vx;
                    hail.vel.y = hail.vel.y + vy;
                    hail.vel.z = hail.vel.z + vz;
                }
                for combos in rel_hail.iter().combinations(2) {
                    let (p1, v1, p2, v2) = (combos[0].pos, combos[0].vel, combos[1].pos, combos[1].vel);
                    if let Some(candidate) = find_intersection_3d(p1, v1, p2, v2) {
                        match intersect {
                            Some(cord) =>{
                                if cord != candidate {
                                    continue 'vz;
                                }
                            }
                            None => intersect = Some(candidate),
                        }
                    }
                }
                match intersect {
                    Some(i) => {
                        println!("found intersection {:?}", i);
                        throw_pos = Some(i);
                        break 'outer
                    }
                    _ => {}
                }

            }
        }
    }
    match throw_pos {
        Some(pos) =>{
            println!("pos.x: {} pos.y: {} pos.z: {}", pos.x, pos.y, pos.z);
            return Some(pos.x + pos.y + pos.z)
        }
        None => return None
    }
}

fn main() {
    let input = include_str!("input.txt");
    let ans_a = part_a(input, (200000000000000.0, 400000000000000.0));
    println!("Part A: {}", ans_a);
    let ans_b = part_b(input);
    println!("Part B: {}", ans_b.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test3d_collision() {
        let p1_0 = Coord::new(19, 13, 30);
        let p2_0 = Coord::new(18, 19, 22);
        let v1 = Coord::new(-2 + 3, 1  - 1,  -2 - 2);
        let v2 = Coord::new(-1 + 3, -1 - 1 , -2 - 2);
        let pos = find_intersection_3d(p1_0, v1, p2_0, v2);
        assert_eq!(pos.unwrap(), Coord::new(24, 13, 10));
    }
}
