#![feature(slice_split_once)]

use std::iter::repeat;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Operation {
    Add(Lens),
    Remove(Vec<u8>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Lens {
    label: Vec<u8>,
    focatl_length: usize,
}

fn parse(input: &str) -> Vec<Operation> {
    input
        .split(',')
        .map(|l| {
            if let Some((lable, _)) = l.split_once('-') {
                Operation::Remove(
                    lable
                        .as_bytes()
                        .iter()
                        .filter(|c| *c != &b'\n')
                        .cloned()
                        .collect(),
                )
            } else {
                let (lable, value) = l.split_once('=').unwrap();
                Operation::Add(Lens {
                    label: lable
                        .as_bytes()
                        .iter()
                        .filter(|c| *c != &b'\n')
                        .cloned()
                        .collect(),
                    focatl_length: value.parse::<usize>().unwrap(),
                })
            }
        })
        .collect()
}

fn compute_hash(label: &Vec<u8>) -> usize {
    label
        .iter()
        .fold(0usize, |acc, c| ((acc + *c as usize) * 17) % 256)
}

fn remove(lenses: &mut Vec<Lens>, label: &Vec<u8>) {
    if let Some(position) = lenses.iter().position(|l| &l.label == label) {
        for i in position..lenses.len().saturating_sub(1) {
            lenses[i] = lenses[i + 1].clone();
        }
        lenses.pop();
    }
}

fn add(lenses: &mut Vec<Lens>, lense: Lens) {
    if let Some(position) = lenses.iter().position(|l| &l.label == &lense.label) {
        lenses[position] = lense;
    } else {
        lenses.push(lense);
    }
}

fn part_1(input: &str) -> String {
    let parts = parse(input);
    let mut boxes: Vec<Vec<Lens>> = repeat(vec![]).take(256).collect();
    for part in parts {
        match part {
            Operation::Add(lens) => {
                let box_index = compute_hash(&lens.label);
                let lenses = boxes.get_mut(box_index).unwrap();
                add(lenses, lens);
            }
            Operation::Remove(label) => {
                let box_index = compute_hash(&label);
                let lenses = boxes.get_mut(box_index).unwrap();
                remove(lenses, &label);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(ii, lens)| (i + 1) * (ii + 1) * lens.focatl_length)
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
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
        let input: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part_1(input), "145");
    }
}
