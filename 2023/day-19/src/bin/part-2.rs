use std::{
    cmp::{max, min},
    collections::HashMap,
};

use winnow::{
    ascii::{dec_uint, multispace0, newline},
    combinator::{alt, eof, preceded, repeat_till0, separated, terminated},
    token::{one_of, take_till, take_until1},
    PResult, Parser,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Workflows(HashMap<String, Vec<Rule>>);

#[derive(Clone, Debug, PartialEq, Eq)]
enum Rule {
    WithCondition(Condition, String),
    WithoutCondition(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Condition {
    G(String, u64),
    S(String, u64),
}

fn condition_parser(input: &mut &str) -> PResult<Condition> {
    let (part, condition, num) = (
        one_of(['x', 'm', 'a', 's']),
        one_of(['>', '<', '=']),
        dec_uint,
    )
        .parse_next(input)?;
    Ok(match condition {
        '<' => Condition::S(part.to_string(), num),
        '>' => Condition::G(part.to_string(), num),
        _ => unreachable!(),
    })
}

fn rule_with_congition_parser(input: &mut &str) -> PResult<Rule> {
    let (condition, destination) = terminated(
        (terminated(condition_parser, ':'), take_till(1.., ',')),
        ',',
    )
    .parse_next(input)?;
    Ok(Rule::WithCondition(condition, destination.to_string()))
}
fn last_rule_parser(input: &mut &str) -> PResult<Rule> {
    let destination = take_until1("}").parse_next(input)?;
    Ok(Rule::WithoutCondition(destination.to_string()))
}

fn rule_parser(input: &mut &str) -> PResult<Rule> {
    alt([rule_with_congition_parser, last_rule_parser]).parse_next(input)
}

fn workflow_parser(input: &mut &str) -> PResult<(String, Vec<Rule>)> {
    let (name, rules) = (
        terminated(take_till(1.., |c| c == '{' || c == '\n'), '{'),
        repeat_till0(rule_parser, '}'),
    )
        .parse_next(input)?;
    Ok((name.to_string(), rules.0))
}

fn workflows_parser(input: &mut &str) -> PResult<Workflows> {
    let workflows: Vec<(String, Vec<Rule>)> =
        separated(0.., workflow_parser, newline).parse_next(input)?;
    Ok(Workflows(HashMap::from_iter(workflows)))
}

fn till_eq_sign_parser(input: &mut &str) -> PResult<()> {
    terminated(take_till(1.., '='), '=')
        .parse_next(input)
        .map(|_| ())
}

fn part_number_parser(input: &mut &str) -> PResult<u64> {
    preceded(till_eq_sign_parser, dec_uint).parse_next(input)
}

fn part_parser(input: &mut &str) -> PResult<Part> {
    let (x, m, a, s) = terminated(
        (
            part_number_parser,
            part_number_parser,
            part_number_parser,
            part_number_parser,
        ),
        '}',
    )
    .parse_next(input)?;
    Ok(Part { x, m, a, s })
}
fn parts_parser(input: &mut &str) -> PResult<Vec<Part>> {
    separated(1.., part_parser, newline).parse_next(input)
}
fn all_parse(input: &mut &str) -> PResult<(Workflows, Vec<Part>)> {
    terminated((workflows_parser, parts_parser), (multispace0, eof)).parse_next(input)
}

fn parse(input: &str) -> (Workflows, Vec<Part>) {
    all_parse.parse(input).unwrap()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Interval {
    lower: u64,
    greater: u64,
}

impl Interval {
    fn new(v1: u64, v2: u64) -> Self {
        Self {
            lower: min(v1, v2),
            greater: max(v1, v2),
        }
    }

    fn intersection(&self, other: &Interval) -> Option<Interval> {
        if self.greater < other.lower || self.lower > other.greater {
            None
        } else {
            Some(Interval {
                lower: max(self.lower, other.lower),
                greater: min(self.greater, other.greater),
            })
        }
    }

    fn to_sum(&self) -> u64 {
        self.greater - self.lower + 1
    }
}

const MIN_VAL: u64 = 1;
const MAX_VAL: u64 = 4000;

impl Default for Interval {
    fn default() -> Self {
        Interval {
            lower: MIN_VAL,
            greater: MAX_VAL,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct PartIntervals {
    x: Interval,
    m: Interval,
    a: Interval,
    s: Interval,
}

impl PartIntervals {
    fn to_sum(&self) -> u64 {
        self.x.to_sum() * self.m.to_sum() * self.a.to_sum() * self.s.to_sum()
    }
    fn set_fild(&mut self, name: &str, interval: &Interval) {
        match name {
            "x" => self.x = interval.clone(),
            "m" => self.m = interval.clone(),
            "a" => self.a = interval.clone(),
            "s" => self.s = interval.clone(),
            _ => unreachable!(),
        };
    }
}

fn solve(workflows: &Workflows, mut part: PartIntervals, name: &String) -> u64 {
    let mut sum = 0;
    if name == "A" {
        return part.to_sum();
    } else if name == "R" {
        return 0;
    }
    let workflow = workflows.0.get(name).unwrap();

    for rule in workflow {
        match rule {
            Rule::WithCondition(condition, next_name) => {
                let part_letter = match condition {
                    Condition::G(part_letter, _) => part_letter,
                    Condition::S(part_letter, _) => part_letter,
                };
                let field = match part_letter.as_str() {
                    "x" => part.x.clone(),
                    "m" => part.m.clone(),
                    "a" => part.a.clone(),
                    "s" => part.s.clone(),
                    _ => unreachable!(),
                };
                match condition {
                    Condition::G(field_name, num) => {
                        let positive = field
                            .intersection(&Interval::new(num + 1, MAX_VAL))
                            .unwrap();
                        let invers = field.intersection(&Interval::new(MIN_VAL, *num)).unwrap();
                        let mut new_part = part.clone();
                        new_part.set_fild(field_name, &positive);
                        sum += solve(workflows, new_part, next_name);
                        part.set_fild(field_name, &invers);
                    }
                    Condition::S(field_name, num) => {
                        let positive = field
                            .intersection(&Interval::new(MIN_VAL, num - 1))
                            .unwrap();
                        let invers = field.intersection(&Interval::new(*num, MAX_VAL)).unwrap();
                        let mut new_part = part.clone();
                        new_part.set_fild(field_name, &positive);
                        sum += solve(workflows, new_part, next_name);
                        part.set_fild(field_name, &invers);
                    }
                }
            }
            Rule::WithoutCondition(next_name) => {
                sum += solve(workflows, part.clone(), next_name)
            }
        }
    }
    sum
}

fn part_2(input: &str) -> String {
    let (workflows, _) = parse(input);
    solve(&workflows, PartIntervals::default(), &"in".to_string()).to_string()
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
    fn it_works() {
        let input: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}\n";
        assert_eq!(part_2(input), "167409079868000");
    }

    #[test]
    fn it_works_2() {
        let res = condition_parser.parse("m>1548").unwrap();
        assert_eq!(res, Condition::G("m".to_string(), 1548));
    }
    #[test]
    fn it_works_3() {
        let res = rule_with_congition_parser.parse("m>1548:R,").unwrap();
        assert_eq!(
            res,
            Rule::WithCondition(Condition::G("m".to_string(), 1548), "R".to_string())
        );
    }
    #[test]
    fn it_works_4() {
        let res = terminated(condition_parser, ':').parse("m>1548:").unwrap();
        assert_eq!(res, Condition::G("m".to_string(), 1548));
    }
    #[test]
    fn it_works_5() {
        use Condition::*;
        use Rule::*;
        let res = workflow_parser
            .parse("px{a<2006:qkq,m>2090:A,rfg}")
            .unwrap();
        assert_eq!(
            res,
            (
                "px".to_string(),
                vec![
                    WithCondition(S("a".to_string(), 2006), "qkq".to_string()),
                    WithCondition(G("m".to_string(), 2090), "A".to_string()),
                    WithoutCondition("rfg".to_string())
                ]
            )
        );
    }
    #[test]
    fn it_works_6() {
        let res = part_parser.parse("{x=1679,m=44,a=2067,s=496}").unwrap();
        assert_eq!(
            res,
            Part {
                x: 1679,
                m: 44,
                a: 2067,
                s: 496
            }
        );
    }
    #[test]
    fn it_works_7() {
        let res = parts_parser
            .parse("{x=1679,m=44,a=2067,s=496}\n{x=2461,m=1339,a=466,s=291}")
            .unwrap();
        assert_eq!(
            res,
            [
                Part {
                    x: 1679,
                    m: 44,
                    a: 2067,
                    s: 496
                },
                Part {
                    x: 2461,
                    m: 1339,
                    a: 466,
                    s: 291
                }
            ]
        );
    }
    #[test]
    fn it_works_8() {
        let res = parts_parser
            .parse(
                "{x=1679,m=44,a=2067,s=496}
{x=1679,m=44,a=2067,s=496}",
            )
            .unwrap();
        assert_eq!(
            res,
            [
                Part {
                    x: 1679,
                    m: 44,
                    a: 2067,
                    s: 496
                },
                Part {
                    x: 1679,
                    m: 44,
                    a: 2067,
                    s: 496
                }
            ]
        );
    }
    #[test]
    fn it_works_9() {
        let res = workflows_parser
            .parse(
                "gd{a>3333:R,R}
hdj{m>838:A,pv}",
            )
            .unwrap();
        use Condition::*;
        use Rule::*;
        assert_eq!(
            res,
            Workflows(HashMap::from_iter(vec![
                (
                    "gd".to_string(),
                    vec![
                        WithCondition(G("a".to_string(), 3333), "R".to_string()),
                        WithoutCondition("R".to_string())
                    ]
                ),
                (
                    "hdj".to_string(),
                    vec![
                        WithCondition(G("m".to_string(), 838), "A".to_string()),
                        WithoutCondition("pv".to_string())
                    ]
                )
            ]))
        );
    }

    #[test]
    fn it_works_10() {
        let i1 = Interval::new(10, 20);
        let i2 = Interval::new(15, 25);
        let ret = Interval::new(15, 20);
        assert_eq!(i1.intersection(&i2), Some(ret));
    }
    #[test]
    fn it_works_11() {
        let i1 = Interval::new(10, 20);
        let i2 = Interval::new(21, 30);
        assert_eq!(i1.intersection(&i2), None);
    }
    #[test]
    fn it_works_12() {
        let i1 = Interval::new(1351, 1);
        let i2 = Interval::new(2771, 4000);
        assert_eq!(i1.intersection(&i2), None);
    }
}
