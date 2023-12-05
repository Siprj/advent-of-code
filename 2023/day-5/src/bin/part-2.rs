use itertools::Itertools;
use std::{
    ops::Range,
    rc::Rc,
};

#[derive(Debug)]
struct Map {
    #[allow(unused)]
    from: String,
    #[allow(unused)]
    to: String,
    range_maps: Vec<MapRange>,
}

#[derive(Debug)]
struct MapRange {
    range: Range<i64>,
    target: i64,
}

#[derive(Debug, Eq, PartialEq)]
enum Translation {
    Full(Range<i64>),
    Partial(Range<i64>, Vec<Range<i64>>),
    None(Range<i64>),
}

impl MapRange {
    fn translate(&self, range: &Range<i64>) -> Translation {
        if self.range.contains(&range.start) {
            if self.range.contains(&(range.end - 1)) {
                let start = self.target + range.start - self.range.start;
                let end = start + range.end - range.start;
                Translation::Full(Range { start, end })
            } else {
                let start = self.target + range.start - self.range.start;
                let end = start + self.range.end - range.start;
                Translation::Partial(
                    Range { start, end },
                    vec![Range {
                        start: self.range.end,
                        end: range.end,
                    }],
                )
            }
        } else if self.range.contains(&(range.end - 1)) {
            let start = self.target;
            let end = start + range.end - self.range.start;
            Translation::Partial(
                Range { start, end },
                vec![Range {
                    start: range.start,
                    end: self.range.start,
                }],
            )
        } else if self.range.start >= range.end || self.range.end <= range.start{
            Translation::None(range.clone())
        } else {
            Translation::Partial(
                Range {
                    start: self.target,
                    end: self.target + self.range.end - self.range.start,
                },
                vec![
                    Range {
                        start: range.start,
                        end: self.range.start,
                    },
                    Range {
                        start: self.range.end,
                        end: range.end,
                    },
                ],
            )
        }
    }
}

fn map_over_ranges(map_ranges: &Vec<MapRange>, range: &Range<i64>) -> Vec<Range<i64>> {
    let mut ret = vec![];
    let mut to_translate: Vec<Range<i64>> = vec![range.clone()];
    for map in map_ranges {
        let mut bla: Vec<Range<i64>> = vec![];
        for r in to_translate.iter() {
            match map.translate(r) {
                Translation::Full(r) => {ret.push(r)},
                Translation::Partial(r, mut rest) => {ret.push(r); bla.append(&mut rest)},
                Translation::None(rest) => {bla.push(rest); },
            }
        }
        to_translate = bla;
    }

    ret.extend(to_translate);
    ret
}

fn merge_ranges(ranges: &mut [Range<i64>]) -> Vec<Range<i64>> {
    let mut ret: Vec<Range<i64>> = Vec::with_capacity(ranges.len());
    ranges.sort_unstable_by(|r1, r2| r1.start.cmp(&r2.start));
    let mut curent_range = ranges[0].clone();
    for range in ranges {
        if range.start <= curent_range.end {
            curent_range.end = range.end;
        } else {
            ret.push(curent_range.clone());
            curent_range = range.clone();
        }
    }
    ret.push(curent_range);
    ret
}

fn parse_seeds(input: &str) -> Vec<Range<i64>> {
    input
        .split_whitespace()
        .tuples()
        .map(|(s, l)| {
            let s = s.parse::<i64>().unwrap();
            let l = l.parse::<i64>().unwrap();
            Range {
                start: s,
                end: s + l,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Input {
    seeds: Vec<Range<i64>>,
    maps: Rc<Vec<Map>>,
}

fn parse_map(input: &str) -> Map {
    let mut lines = input.lines();
    let section_header = lines.next().unwrap();
    let nameing_part = section_header.split_whitespace().next().unwrap();
    let nameing_part: Vec<String> = nameing_part.split('-').map(|s| s.to_string()).collect();
    let range_maps = lines
        .map(|str| {
            let parts: Vec<&str> = str.split_whitespace().collect();
            let target = parts[0].parse().unwrap();
            let start = parts[1].parse().unwrap();
            let len: i64 = parts[2].parse().unwrap();
            MapRange {
                range: Range {
                    start,
                    end: start + len,
                },
                target,
            }
        })
        .collect();

    Map {
        from: nameing_part[0].clone(),
        to: nameing_part[2].clone(),
        range_maps,
    }
}

fn parse_input(input: &str) -> Input {
    let mut sections = input.split("\n\n");
    let seeds_section = sections.next().unwrap();
    let mut seeds = seeds_section.split(':');
    seeds.next(); // drop the "seeds: " prefix
    let seeds = parse_seeds(seeds.next().unwrap());

    let maps = sections.map(parse_map).collect();

    Input {
        seeds,
        maps: Rc::new(maps),
    }
}

fn part_2(input: &str) -> String {
    let data = parse_input(input);
    let mut ranges: Vec<Range<i64>> = data.seeds.clone();
    for map in Rc::clone(&data.maps).iter() {
        ranges = merge_ranges(&mut ranges);
        ranges = ranges.iter().flat_map(|r| {map_over_ranges(&map.range_maps,r)}).collect();
    }
    let r = ranges
        .iter()
        .min_by(|r1, r2| r1.start.cmp(&r2.start))
        .unwrap();
    r.start.to_string()
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
    fn map_translate() {
        let map_range = MapRange {
            range: Range { start: 10, end: 15 },
            target: 30,
        };
        let range = Range { start: 10, end: 15 };
        assert_eq!(map_range.translate(&range), Translation::Full(30i64..35));
        let range = Range { start: 10, end: 17 };
        assert_eq!(
            map_range.translate(&range),
            Translation::Partial(30i64..35, vec![15i64..17])
        );
        let range = Range { start: 5, end: 11 };
        assert_eq!(
            map_range.translate(&range),
            Translation::Partial(30i64..31, vec![5i64..10])
        );
        let range = Range { start: 5, end: 10 };
        assert_eq!(map_range.translate(&range), Translation::None(5i64..10));
        let range = Range { start: 5, end: 16 };
        assert_eq!(
            map_range.translate(&range),
            Translation::Partial(30i64..35, vec![5i64..10, 15i64..16])
        );
        let range = Range { start: 15, end: 19 };
        assert_eq!(
            map_range.translate(&range),
            Translation::None(15i64..19)
        );
    }

    #[test]
    fn it_works() {
        let input: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part_2(input), "46");
    }
}
