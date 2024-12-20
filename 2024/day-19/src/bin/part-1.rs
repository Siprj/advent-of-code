use std::collections::{BinaryHeap, HashMap};

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (available, patterns) = input.trim().split_once("\n\n").unwrap();
    let available = available.split(", ").collect();
    let patterns = patterns.split_whitespace().collect();
    (available, patterns)
}

#[derive(Debug)]
struct Node {
    next: HashMap<char, Box<Node>>,
}

impl Node {
    fn new() -> Self {
        Node {
            next: HashMap::new(),
        }
    }
}

fn make_machine(available: &[&str]) -> HashMap<char, Box<Node>> {
    let mut machine: HashMap<char, Box<Node>> = HashMap::new();
    for a in available.iter() {
        let mut char_iter = a.chars();
        let first_char: char = char_iter.next().unwrap();
        let entry = machine.entry(first_char);
        let mut node = entry.or_insert(Box::new(Node::new()));
        for c in char_iter {
            let entry = node.next.entry(c);
            node = entry.or_insert(Box::new(Node::new()));
        }
        node.next.entry('#').or_insert(Box::new(Node::new()));
    }
    machine
}

#[derive(Debug)]
struct Token<'a> {
    index: usize,
    mach: &'a Box<Node>,
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index)
    }
}

impl<'a> Eq for Token<'a> {}

impl<'a> PartialOrd for Token<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Token<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

fn part_1(input: &str) -> String {
    let (available, patterns) = parse(input);
    let machine = Box::new(Node {
        next: make_machine(&available),
    });

    let mut count_designes: i32 = 0;
    'outer: for pattern in patterns {
        let pattern: Vec<char> = pattern.chars().collect();
        //let mut node = &machine;
        let mut max_heap = BinaryHeap::with_capacity(1000);
        max_heap.push(Token {
            index: 0,
            mach: &machine,
        });
        let pattern_len = pattern.len();
        let mut k = 0;
        while let Some(t) = max_heap.pop() {
            k += 1;
            if k > 1000 {
                continue 'outer;
            }

            if t.index == pattern_len {
                if t.mach.next.contains_key(&'#') {
                    count_designes += 1;
                    continue 'outer;
                }
            } else {
                if let Some(next) = t.mach.next.get(&pattern[t.index]) {
                    max_heap.push(Token {
                        index: t.index + 1,
                        mach: next,
                    });
                }

                if let Some(_end) = t.mach.next.get(&'#') {
                    max_heap.push(Token {
                        index: t.index,
                        mach: &machine,
                    });
                }
            }
        }
    }
    count_designes.to_string()
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
        let input: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(part_1(input), "6");
    }

    //    #[test]
    //    fn it_works2() {
    //        let input: &str = "rug, uugbb, bubbbr, bbubw, wwb, wbw, gwbbuw, ubg, ruwug, bbwuww, wubwrw, ug, ugu, ggrbg, ruw, rbur, uuubbuu, wb, wurrbgr, rwruwwuu, rwg, rwbb, bwugugg, bur, ggwub, brgbr, guuu, wgg, bbw, ugrbrb, ggrw, wbgu, rwgbg, ggwu, wurwuw, gruub, gugw, bgu, wrr, rbrg, bww, uggwrr, rgg, uugb, bbrwuru, rw, urbu, ggggg, ru, wrrgwrrb, uurrgur, bgrw, ubuwr, gwwgug, bggb, uwub, gwu, wrwbgw, rrbb, uuwrggb, uuugw, bbbubg, bg, ggr, rgw, ugburuw, ggbbww, rrgbru, wurg, wugu, rbuu, wgrg, ubugu, uuwuur, ur, wgbbg, wubu, gbrrwgu, bwbu, wbgguwg, rrwbwru, buggggg, bwu, uw, grubg, bggbw, ugwwgwu, rbubgwg, wgwuu, rrr, uubwwgrr, bbwguw, uuru, wgww, grbwu, gbgww, uuu, gruwwg, urgw, ubu, ggbgb, buwur, bbgb, bub, wbru, gbug, wgrruub, wrwwru, wuwgg, uwg, uuwgw, rrruwb, rbr, grrrugwb, rbgb, wwbr, ggbugg, ggg, buu, uwr, guug, wg, wbwu, wgb, rbbb, rub, wru, bubr, rgruwrgr, gwr, urru, wuw, uuwg, bbr, ruwg, urwgu, bwbugbb, wgbgrwgg, grwb, rwgg, uugg, uww, w, urwgrgb, gw, ubrb, grbrrbrw, wgwgrr, rggurb, buuww, bgrrb, wbrg, ggburu, rgwgb, wrw, bw, wubbb, bbwww, ubbg, bbg, ruururru, rbg, wgug, gur, rrugw, gwurbg, gr, bwwgrr, gub, wrgwrw, gbrrb, rr, rgbu, bbrur, ubgbu, wuwrw, guub, guu, uwbww, gguuww, ubuw, uuggu, wgwg, grwrr, bwg, uwgrb, grwu, gwbw, rwu, buubrb, ggbubw, wrgw, wwu, uuubwrgb, urwgbwu, rwggwrg, uuuw, gbrwb, bbuu, urgbgb, www, guw, uuw, wbubg, brw, gubruu, uwrg, bbbggg, bwr, wgbg, wguw, rbbgruru, uubgggr, ugw, grgugr, uugrrbb, rwgwg, ubgu, burgwgbw, gwrubbu, bwuug, bburur, burwu, wbb, ub, wr, ggubww, grw, ww, buw, urbru, gbgr, burwgur, ugr, gwwbgb, urr, wuwg, bbubrrgu, wuuwb, brrwg, g, gbgurrw, rgu, ubb, urggug, wub, gwubu, bgrr, wrbwwb, guuuuug, rrb, brugwu, wgrgwu, bru, wrbgugur, ubww, uur, uuub, wubwrb, uubggur, bbu, urb, rrggwuw, brgw, urbrrg, uwuur, bubw, bbwuu, gbu, rrg, rrgbw, gbr, wbu, ubrw, rww, ubw, wugr, rrur, bggu, uurrbwr, br, ggu, bgww, gwbbubgu, grwbw, uu, wguwg, urwubg, rruwg, wur, rgbwg, wbrgbg, ugrr, wbbggu, gww, bugb, wrbr, grurrg, gbrgwb, rur, gubbr, gbb, gggubwgu, gwbbbwru, rbbbu, rg, gwg, bgg, rgbg, uubbrwb, uwur, rwr, uwu, wbg, wwwbgurr, grb, gbg, rugg, wgub, bb, bgb, ubwbrww, gugbr, wwgwwwug, brb, rbb, uwgg, ubbbw, rwwr, urrw, ubuuub, wbub, ugbrr, ubrr, bgwr, gwb, wuuw, ggbuwb, wug, wgugr, urgrb, uwbbwr, gwgb, uug, gbur, wwr, gug, rgbubw, ruu, gbbw, grg, ggruwgb, urwggu, rrw, rgguuu, bgwg, uub, rbu, bu, grrr, r, ugb, bbrb, gwwbr, bwwur, wwg, grurrgb, wrguuu, rgggw, ugbwg, rwb, wubgw, grrww, uwb, bgw, bggrw, rgr, rbwg, buugbbb, bgr, gbgrgr, wbr, wgr, wbbr, gg, gru, urg, bguu, bbwbgur, bburgw, ubr, wrg, gu, rbw, rwbrur, bwgru, ubwrugb, uggubw, rgbr, bbgg, wrb, wwuug, bwbuw, rb, gwugbr, uuwrw, u, rburg, wguwgwu, bwrru, uguwgb, wuu, ugg, bgrg, wrrw, brg, rwwru, buuwg, ubuuwwr, ruwr, bbb, guwuwuug, rrbgg, ugwub, rrwwugb, bug, wgw, urw, brr, bwuwrw, bbug, rru, brbubb, bwb, ggbr, rgb, gwrwb, grbgw, rrwuruw, wruuwgrb, brruubu, bgwbu, grr, ubru, rbrr, wubwuub, grbr
    //
    //bwbbrrgrrbrggubuggwgguguburbbgbgrruggugbggggb";
    //        assert_eq!(part_1(input), "0");
    //    }
}
