use std::collections::{HashMap, HashSet};

use petgraph::{graph::UnGraph, stable_graph::NodeIndex};
use rustworkx_core::connectivity::stoer_wagner_min_cut;


fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines() {
        let (from, rest) = line.split_once(':').unwrap();
        for to in rest.trim().split(" ") {
            graph.entry(from.to_string()).or_default().insert(to.to_string());
            graph.entry(to.to_string()).or_default().insert(from.to_string());
        }
    }
    graph
}

fn to_petgraph(graph: &HashMap<String, HashSet<String>>) -> UnGraph::<String, ()>{
    let mut visited: HashSet<String> = HashSet::new();
    println!("graph cool_name {{");
    let mut new_graph: UnGraph::<String, ()> = UnGraph::default();
    let node_index: HashMap<String, NodeIndex> = graph.iter().map(|node| (node.0.clone(), new_graph.add_node(node.0.clone()))).collect();
    for (node, to_nodes) in graph {
        visited.insert(node.clone());
        for to_node in to_nodes {
            if !visited.contains(to_node) {
                new_graph.add_edge(*node_index.get(node).unwrap(), *node_index.get(to_node).unwrap(), ());
            }
        }
    }
    new_graph
}

fn part_1(input: &str) -> String {
    let graph = parse(input);
    let pet_graph = to_petgraph(&graph);
    let min_cut: rustworkx_core::Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&pet_graph, |_| Ok(1));
    let min_cut = min_cut.unwrap().unwrap();

    ((pet_graph.node_count()- min_cut.1.len()) * min_cut.1.len()).to_string()
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
        let input: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!(part_1(input), "54");
    }
}
