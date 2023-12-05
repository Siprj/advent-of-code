use std::rc::Rc;

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
    target: u64,
}

impl Map {
    fn translate(&self, from: u64) -> u64 {
        if let Some(range) = self.ranges.iter().find(|r| from >= r.start && from < r.end) {
            range.target + from - range.start
        } else {
            from
        }
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    maps: Rc<Vec<Map>>,
}

fn parse_map(input: &str) -> Map {
    let mut lines = input.lines();
    let section_header = lines.next().unwrap();
    let nameing_part = section_header.split_whitespace().next().unwrap();
    let nameing_part: Vec<String> = nameing_part.split('-').map(|s| s.to_string()).collect();
    let ranges = lines
        .map(|str| {
            let parts: Vec<&str> = str.split_whitespace().collect();
            let target = parts[0].parse().unwrap();
            let start = parts[1].parse().unwrap();
            let len: u64 = parts[2].parse().unwrap();
            Range {
                start,
                target,
                end: start + len,
            }
        })
        .collect();

    Map {
        from: nameing_part[0].clone(),
        to: nameing_part[2].clone(),
        ranges,
    }
}

fn parse_input(input: &str) -> Input {
    let mut sections = input.split("\n\n");
    let seeds_section = sections.next().unwrap();
    let mut seeds = seeds_section.split(':');
    seeds.next(); // drop the "seeds: " prefix
    let seeds = seeds
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let maps = sections.map(parse_map).collect();

    Input {
        seeds,
        maps: Rc::new(maps),
    }
}

fn part_1(input: &str) -> String {
    let data = parse_input(input);
    let smallest: u64 = data
        .seeds
        .iter()
        .map(|seed| {
            let mut current: u64 = *seed;
            for map in Rc::clone(&data.maps).iter() {
                current = map.translate(current);
            }
            current
        })
        .min()
        .unwrap();
    smallest.to_string()
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
        assert_eq!(part_1(input), "35");
    }
}
