use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> Vec<(&str, &str)> {
    input.lines().map(|l| l.split_once('-').unwrap()).collect()
}

fn part_1(input: &str) -> String {
    let connections = parse(input);

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for connection in connections.iter() {
        graph.entry(connection.0).or_default().insert(connection.1);
        graph.entry(connection.1).or_default().insert(connection.0);
    }

    let mut sets: HashSet<Vec<&str>> = HashSet::new();

    for (k, next_nodes) in graph.iter() {
        for next_node in next_nodes.iter() {
            for next_node2 in graph.get(next_node).unwrap() {
                if graph.get(next_node2).unwrap().contains(k) {
                    let mut new_set: Vec<&str> = vec![k, next_node, next_node2];
                    new_set.sort();
                    sets.insert(new_set);
                }
            }
        }
    }

    let mut sum = 0;

    for set in sets.iter() {
        if set.iter().any(|m| m.starts_with('t')) {
            sum += 1;
        }
    }

    sum.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!(part_1(input), "7");
    }
}
