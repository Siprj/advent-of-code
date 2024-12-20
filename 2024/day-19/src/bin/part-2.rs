use std::collections::HashMap;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (available, patterns) = input.trim().split_once("\n\n").unwrap();
    let available = available.split(", ").collect();
    let patterns = patterns.split_whitespace().collect();
    (available, patterns)
}

#[derive(Debug)]
struct Node {
    nodes: HashMap<char, Node>,
    is_leaf: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            is_leaf: false,
        }
    }
}

#[derive(Debug)]
struct PrefixTree {
    root: Node,
}

impl PrefixTree {
    fn new() -> Self {
        PrefixTree {
            root: Node {
                nodes: HashMap::new(),
                is_leaf: false,
            },
        }
    }

    fn build_from_list(available: &[&str]) -> PrefixTree {
        let mut prefix_tree = PrefixTree::new();
        for prefix in available.iter() {
            prefix_tree.insert(prefix);
        }
        prefix_tree
    }

    fn insert(&mut self, str: &str) {
        let mut node = &mut self.root;
        for c in str.chars() {
            let entry = node.nodes.entry(c);
            entry.or_insert(Node::new());
            node = node.nodes.get_mut(&c).unwrap();
        }
        node.is_leaf = true;
    }

    fn contains_prefix(&self, str: &str) -> bool {
        let mut node = &self.root;
        for c in str.chars() {
            if let Some(next) = node.nodes.get(&c) {
                node = next;
            } else {
                return false;
            }
        }
        node.is_leaf
    }

    fn get_prefixes<'a>(&self, str: &'a str) -> Vec<&'a str> {
        let mut ret: Vec<&str> = Vec::new();
        for i in 1..=str.len() {
            if self.contains_prefix(&str[0..i]) {
                ret.push(&str[0..i]);
            }
        }
        ret
    }
}

fn count_posible<'a>(str: &'a str, index: usize, prefix_tree: &PrefixTree, cache: &mut HashMap<&'a str, usize>) -> usize {
    if index >= str.len() {
        return 1;
    }

    let mut sum = 0;
    let prefixes: Vec<&str> = prefix_tree.get_prefixes(&str[index..]);
    for prefix in prefixes.iter() {
        let next_index = index + prefix.len();
        if let Some(v) = cache.get(&str[next_index..]) {
            sum += v;
        } else {
            let ret = count_posible(str, next_index, prefix_tree, cache);
            sum += ret;
        }
    }
    cache.insert(&str[index..], sum);
    sum
}

fn part_2(input: &str) -> String {
    let (available, patterns) = parse(input);
    let prefix_tree = PrefixTree::build_from_list(&available);

    let mut sum = 0;
    for pattern in patterns.iter() {
        sum += count_posible(pattern, 0, &prefix_tree, &mut HashMap::new());
    }
    sum.to_string()
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
        let input: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        assert_eq!(part_2(input), "16");
    }

    #[test]
    fn it_works2() {
        let input: &str = "rug, uugbb, bubbbr, bbubw, wwb, wbw, gwbbuw, ubg, ruwug, bbwuww, wubwrw, ug, ugu, ggrbg, ruw, rbur, uuubbuu, wb, wurrbgr, rwruwwuu, rwg, rwbb, bwugugg, bur, ggwub, brgbr, guuu, wgg, bbw, ugrbrb, ggrw, wbgu, rwgbg, ggwu, wurwuw, gruub, gugw, bgu, wrr, rbrg, bww, uggwrr, rgg, uugb, bbrwuru, rw, urbu, ggggg, ru, wrrgwrrb, uurrgur, bgrw, ubuwr, gwwgug, bggb, uwub, gwu, wrwbgw, rrbb, uuwrggb, uuugw, bbbubg, bg, ggr, rgw, ugburuw, ggbbww, rrgbru, wurg, wugu, rbuu, wgrg, ubugu, uuwuur, ur, wgbbg, wubu, gbrrwgu, bwbu, wbgguwg, rrwbwru, buggggg, bwu, uw, grubg, bggbw, ugwwgwu, rbubgwg, wgwuu, rrr, uubwwgrr, bbwguw, uuru, wgww, grbwu, gbgww, uuu, gruwwg, urgw, ubu, ggbgb, buwur, bbgb, bub, wbru, gbug, wgrruub, wrwwru, wuwgg, uwg, uuwgw, rrruwb, rbr, grrrugwb, rbgb, wwbr, ggbugg, ggg, buu, uwr, guug, wg, wbwu, wgb, rbbb, rub, wru, bubr, rgruwrgr, gwr, urru, wuw, uuwg, bbr, ruwg, urwgu, bwbugbb, wgbgrwgg, grwb, rwgg, uugg, uww, w, urwgrgb, gw, ubrb, grbrrbrw, wgwgrr, rggurb, buuww, bgrrb, wbrg, ggburu, rgwgb, wrw, bw, wubbb, bbwww, ubbg, bbg, ruururru, rbg, wgug, gur, rrugw, gwurbg, gr, bwwgrr, gub, wrgwrw, gbrrb, rr, rgbu, bbrur, ubgbu, wuwrw, guub, guu, uwbww, gguuww, ubuw, uuggu, wgwg, grwrr, bwg, uwgrb, grwu, gwbw, rwu, buubrb, ggbubw, wrgw, wwu, uuubwrgb, urwgbwu, rwggwrg, uuuw, gbrwb, bbuu, urgbgb, www, guw, uuw, wbubg, brw, gubruu, uwrg, bbbggg, bwr, wgbg, wguw, rbbgruru, uubgggr, ugw, grgugr, uugrrbb, rwgwg, ubgu, burgwgbw, gwrubbu, bwuug, bburur, burwu, wbb, ub, wr, ggubww, grw, ww, buw, urbru, gbgr, burwgur, ugr, gwwbgb, urr, wuwg, bbubrrgu, wuuwb, brrwg, g, gbgurrw, rgu, ubb, urggug, wub, gwubu, bgrr, wrbwwb, guuuuug, rrb, brugwu, wgrgwu, bru, wrbgugur, ubww, uur, uuub, wubwrb, uubggur, bbu, urb, rrggwuw, brgw, urbrrg, uwuur, bubw, bbwuu, gbu, rrg, rrgbw, gbr, wbu, ubrw, rww, ubw, wugr, rrur, bggu, uurrbwr, br, ggu, bgww, gwbbubgu, grwbw, uu, wguwg, urwubg, rruwg, wur, rgbwg, wbrgbg, ugrr, wbbggu, gww, bugb, wrbr, grurrg, gbrgwb, rur, gubbr, gbb, gggubwgu, gwbbbwru, rbbbu, rg, gwg, bgg, rgbg, uubbrwb, uwur, rwr, uwu, wbg, wwwbgurr, grb, gbg, rugg, wgub, bb, bgb, ubwbrww, gugbr, wwgwwwug, brb, rbb, uwgg, ubbbw, rwwr, urrw, ubuuub, wbub, ugbrr, ubrr, bgwr, gwb, wuuw, ggbuwb, wug, wgugr, urgrb, uwbbwr, gwgb, uug, gbur, wwr, gug, rgbubw, ruu, gbbw, grg, ggruwgb, urwggu, rrw, rgguuu, bgwg, uub, rbu, bu, grrr, r, ugb, bbrb, gwwbr, bwwur, wwg, grurrgb, wrguuu, rgggw, ugbwg, rwb, wubgw, grrww, uwb, bgw, bggrw, rgr, rbwg, buugbbb, bgr, gbgrgr, wbr, wgr, wbbr, gg, gru, urg, bguu, bbwbgur, bburgw, ubr, wrg, gu, rbw, rwbrur, bwgru, ubwrugb, uggubw, rgbr, bbgg, wrb, wwuug, bwbuw, rb, gwugbr, uuwrw, u, rburg, wguwgwu, bwrru, uguwgb, wuu, ugg, bgrg, wrrw, brg, rwwru, buuwg, ubuuwwr, ruwr, bbb, guwuwuug, rrbgg, ugwub, rrwwugb, bug, wgw, urw, brr, bwuwrw, bbug, rru, brbubb, bwb, ggbr, rgb, gwrwb, grbgw, rrwuruw, wruuwgrb, brruubu, bgwbu, grr, ubru, rbrr, wubwuub, grbr

    bwbbrrgrrbrggubuggwgguguburbbgbgrruggugbggggb";
        assert_eq!(part_2(input), "0");
    }
}
