use std::collections::{HashMap, HashSet};

use day_8::{parse, Point};

fn part_2(input: &str) -> String {
    let boxes = parse(input);
    let pairs = get_shortest_distances(&boxes);

    let mut point_to_circuits: HashMap<Point, usize> = HashMap::new();
    let mut circuit_to_points: HashMap<usize, HashSet<Point>> = HashMap::new();
    let mut circuit_number = 0;
    let mut remaning_boxes: HashSet<Point> = boxes.iter().cloned().collect();
    let mut res_p1 = Point { x: 0, y: 0, z: 0 };
    let mut res_p2 = Point { x: 0, y: 0, z: 0 };

    for (_, p1, p2) in pairs {
        if remaning_boxes.is_empty() && circuit_to_points.len() == 1 {
            break;
        }
        res_p1 = p1.clone();
        res_p2 = p2.clone();
        remaning_boxes.remove(&p1);
        remaning_boxes.remove(&p2);
        match (
            point_to_circuits.get(&p1).copied(),
            point_to_circuits.get(&p2).copied(),
        ) {
            (None, None) => {
                point_to_circuits.insert(p1.clone(), circuit_number);
                point_to_circuits.insert(p2.clone(), circuit_number);
                let mut set = HashSet::new();
                set.insert(p1);
                set.insert(p2);
                circuit_to_points.insert(circuit_number, set);
                circuit_number += 1;
            }
            (None, Some(p2_circuit)) => {
                point_to_circuits.insert(p1.clone(), p2_circuit);
                circuit_to_points.get_mut(&p2_circuit).unwrap().insert(p1);
            }
            (Some(p1_circuit), None) => {
                point_to_circuits.insert(p2.clone(), p1_circuit);
                circuit_to_points.get_mut(&p1_circuit).unwrap().insert(p2);
            }
            (Some(p1_circuit), Some(p2_circuit)) => {
                if p1_circuit != p2_circuit {
                    let p2_points = circuit_to_points.remove(&p2_circuit).unwrap();
                    for p2 in p2_points.iter() {
                        *point_to_circuits.get_mut(p2).unwrap() = p1_circuit;
                    }

                    let p1_points = circuit_to_points.get_mut(&p1_circuit).unwrap();
                    for p2 in p2_points {
                        p1_points.insert(p2);
                    }
                }
            }
        }
    }
    (res_p1.x * res_p2.x).to_string()
}

fn get_shortest_distances(points: &[Point]) -> Vec<(isize, Point, Point)> {
    let mut res: Vec<(isize, Point, Point)> = vec![];

    for (index, p1) in points.iter().enumerate() {
        let from = index + 1;
        for p2 in points[from..].iter() {
            let distance = p1.distance_squared(p2);
            res.push((distance, p1.clone(), p2.clone()));
        }
    }
    res.sort();
    res
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part_2(input), "25272");
    }
}
