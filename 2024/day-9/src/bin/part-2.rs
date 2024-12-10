use std::{cmp::Reverse, collections::{BinaryHeap, VecDeque}, iter};

use itertools::Itertools;

fn parse(input: &str) -> VecDeque<(usize, (u8, u8))> {
    input.as_bytes().iter().map(|c| (c - 48) ).chain(iter::once(0)).tuples().enumerate().collect()
}

#[allow(clippy::same_item_push)]
fn part_2(input: &str) -> String {
    let segments = parse(input);
    let mut result: Vec<usize> = Vec::with_capacity(segments.len() * 10);
    let mut min_heaps: Vec<BinaryHeap<Reverse<usize>>> = (0..10).map(|_|BinaryHeap::new()).collect();

    let mut position: usize = 0;
    for (v, (taken, free)) in segments.iter() {
        for _ in 0..*taken {
            result.push(*v);
            position += 1;
        }
        min_heaps[*free as usize].push(Reverse(position));
        for _ in 0..*free {
            result.push(0);
            position += 1;
        }
    }

    let mut segment_index : i64 = result.len() as i64 - 1;
    for (v, (used, free)) in segments.iter().rev() {
        let mut heap_index: usize = usize::MAX;
        let mut min_index = usize::MAX;
        for (i, heap) in min_heaps.iter().enumerate().skip(*used as usize) {
            //println!("{i} :::: {heap:?}");
            if let Some(Reverse(index)) = heap.peek(){
               if *index < min_index && segment_index - *free as i64 > *index as i64 {
                   heap_index = i;
                   min_index = *index;
               }
            }
        }

        if heap_index != usize::MAX {
            for i in 0..*used as usize {
                result[segment_index as usize - i - *free as usize] = 0;
            }
            for i in 0..*used as usize {
                result[min_index + i] = *v;
            }
            min_heaps[heap_index].pop();
            let diff = heap_index - *used as usize;
            if diff != 0 {
                min_heaps[diff].push(Reverse(min_index + *used as usize));
            }
        }

        segment_index -= *used as i64 + *free as i64;
    }

    let mut check_sum = 0;
    for (i, v) in result.iter().enumerate() {
        check_sum += i * v;
    }
    check_sum.to_string()

}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input.trim());
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "2333133121414131402";
        assert_eq!(part_2(input), "2858");
    }
    #[test]
    fn it_works2() {
        let input: &str = "12345";
        assert_eq!(part_2(input), "132");
    }
}
