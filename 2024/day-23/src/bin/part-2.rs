use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> Vec<(&str, &str)> {
    input.lines().map(|l| l.split_once('-').unwrap()).collect()
}

fn bron_kerbosch<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    sets: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        sets.push(r);
    } else if !p.is_empty() {
        let nodes: HashSet<&str> = p.iter().cloned().collect();
        for node in nodes.iter() {
            let neighbours: &HashSet<&str> = graph.get(node).unwrap();
            let mut to_add: HashSet<&str> = HashSet::new();
            to_add.insert(*node);
            bron_kerbosch(
                graph,
                r.union(&to_add).cloned().collect(),
                p.intersection(neighbours).cloned().collect(),
                x.intersection(neighbours).cloned().collect(),
                sets,
            );
            p.remove(node);
            x.insert(*node);
        }
    }
}

fn part_2(input: &str) -> String {
    let connections = parse(input);

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for connection in connections.iter() {
        graph.entry(connection.0).or_default().insert(connection.1);
        graph.entry(connection.1).or_default().insert(connection.0);
    }

    let mut sets: Vec<HashSet<&str>> = Vec::new();
    bron_kerbosch(
        &graph,
        HashSet::new(),
        graph.keys().copied().collect(),
        HashSet::new(),
        &mut sets,
    );

    let bigest = sets.iter().max_by_key(|v| v.len()).unwrap();
    let mut ret = bigest.iter().copied().collect::<Vec<&str>>();
    ret.sort();
    ret.join(",")
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
        assert_eq!(part_2(input), "co,de,ka,ta");
    }
}
