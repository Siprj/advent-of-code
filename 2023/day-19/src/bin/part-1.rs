use std::collections::HashMap;

use winnow::{
    ascii::{dec_uint, newline, multispace0},
    combinator::{alt, eof, preceded, repeat_till0, separated, terminated},
    token::{one_of, take_till, take_until1},
    PResult, Parser,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
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
    G(String, u32),
    E(String, u32),
    S(String, u32),
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
        '=' => Condition::E(part.to_string(), num),
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

fn part_number_parser(input: &mut &str) -> PResult<u32> {
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
    terminated((workflows_parser, parts_parser), (multispace0 ,eof)).parse_next(input)
}

fn parse(input: &str) -> (Workflows, Vec<Part>) {
    all_parse.parse(input).unwrap()
}

fn part_to_number(s: &str, part: &Part) -> u32 {
    match s {
        "x" => part.x,
        "m" => part.m,
        "a" => part.a,
        "s" => part.s,
        _ => unreachable!(),
    }
}

fn resolve_workflows(workflows: &Workflows, part: &Part) -> bool {
    let mut workflow = workflows.0.get("in").unwrap();
    loop {
        for rule in workflow {
            match rule {
                Rule::WithCondition(condition, target) => match condition {
                    Condition::G(s, n) => {
                        if &part_to_number(s, part) > n {
                            if target == "R" {
                                return false;
                            } else if target == "A" {
                                return true;
                            } else {
                                workflow = workflows.0.get(target).unwrap();
                                break;
                            }
                        }
                    }
                    Condition::E(s, n) => {
                        if &part_to_number(s, part) == n {
                            if target == "R" {
                                return false;
                            } else if target == "A" {
                                return true;
                            } else {
                                workflow = workflows.0.get(target).unwrap();
                                break;
                            }
                        }
                    }
                    Condition::S(s, n) => {
                        if &part_to_number(s, part) < n {
                            if target == "R" {
                                return false;
                            } else if target == "A" {
                                return true;
                            } else {
                                workflow = workflows.0.get(target).unwrap();
                                break;
                            }
                        }
                    }
                },
                Rule::WithoutCondition(target) => {
                    if target == "R" {
                        return false;
                    } else if target == "A" {
                        return true;
                    } else {
                        workflow = workflows.0.get(target).unwrap();
                    }
                }
            }
        }
    }
}

fn part_1(input: &str) -> String {
    let (workflows, parts) = parse(input);
    let mut accepted = vec![];
    for part in parts {
        if resolve_workflows(&workflows, &part) {
            accepted.push(part);
        } else {
        }
    }
    let mut sum = 0u32;
    for acc in accepted {
        sum += acc.x;
        sum += acc.m;
        sum += acc.a;
        sum += acc.s;
    }
    sum.to_string()
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
        assert_eq!(part_1(input), "19114");
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
}
