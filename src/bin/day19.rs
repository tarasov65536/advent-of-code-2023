use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u64},
    character::complete::{char, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("day19.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let (ws, parts) = parse_input(input).unwrap().1;
    parts
        .iter()
        .filter_map(|p| (ws.apply(p) == Destination::Accepted).then_some(p.total()))
        .sum::<u64>()
        .to_string()
}

fn part2(input: &str) -> String {
    let (ws, _) = parse_input(input).unwrap().1;
    let proto = PartProto::new(
        Range::new(1, 4000),
        Range::new(1, 4000),
        Range::new(1, 4000),
        Range::new(1, 4000),
    );

    ws.predict(&proto)
        .iter()
        .filter_map(|(p, d)| (**d == Destination::Accepted).then_some(p.score()))
        .sum::<u64>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, (WorkflowSet, Vec<Part>)> {
    separated_pair(
        WorkflowSet::parse,
        pair(line_ending, line_ending),
        separated_list1(line_ending, Part::parse),
    )(input)
}

#[derive(Clone, Copy)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn new(min: u64, max: u64) -> Self {
        Self { min, max }
    }

    fn len(&self) -> u64 {
        self.max + 1 - self.min
    }

    fn split(&self, value: u64) -> (Option<Self>, Option<Self>) {
        if value < self.min {
            (
                None,
                Some(Self {
                    min: self.min,
                    max: self.max,
                }),
            )
        } else if self.max < value {
            (
                Some(Self {
                    min: self.min,
                    max: self.max,
                }),
                None,
            )
        } else {
            (
                Some(Self {
                    min: self.min,
                    max: value - 1,
                }),
                Some(Self {
                    min: value,
                    max: self.max,
                }),
            )
        }
    }
}

#[derive(Clone, Copy)]
struct PartProto {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartProto {
    fn new(x: Range, m: Range, a: Range, s: Range) -> Self {
        Self { x, m, a, s }
    }

    fn score(&self) -> u64 {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    fn split_by_category(
        &self,
        category: &Category,
        value: u64,
    ) -> (Option<PartProto>, Option<PartProto>) {
        match category {
            Category::X => {
                let (l, r) = self.x.split(value);
                (
                    l.map(|x| Self { x, ..*self }),
                    r.map(|x| Self { x, ..*self }),
                )
            }
            Category::M => {
                let (l, r) = self.m.split(value);
                (
                    l.map(|m| Self { m, ..*self }),
                    r.map(|m| Self { m, ..*self }),
                )
            }
            Category::A => {
                let (l, r) = self.a.split(value);
                (
                    l.map(|a| Self { a, ..*self }),
                    r.map(|a| Self { a, ..*self }),
                )
            }
            Category::S => {
                let (l, r) = self.s.split(value);
                (
                    l.map(|s| Self { s, ..*self }),
                    r.map(|s| Self { s, ..*self }),
                )
            }
        }
    }
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            delimited(
                char('{'),
                tuple((
                    preceded(tag("x="), u64),
                    char(','),
                    preceded(tag("m="), u64),
                    char(','),
                    preceded(tag("a="), u64),
                    char(','),
                    preceded(tag("s="), u64),
                )),
                char('}'),
            ),
            |(x, _, m, _, a, _, s)| Self { x, m, a, s },
        )(input)
    }

    fn category_value(&self, category: &Category) -> u64 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn total(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

enum Category {
    X,
    M,
    A,
    S,
}

enum WorkflowRule {
    Gt {
        category: Category,
        value: u64,
        next: Destination,
    },
    Lt {
        category: Category,
        value: u64,
        next: Destination,
    },
    Dest(Destination),
}

impl WorkflowRule {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(
                tuple((
                    alt((char('x'), char('m'), char('a'), char('s'))),
                    alt((char('<'), char('>'))),
                    u64,
                    char(':'),
                    alpha1,
                )),
                |(cat, op, value, _, next)| {
                    let category = match cat {
                        'x' => Category::X,
                        'm' => Category::M,
                        'a' => Category::A,
                        's' => Category::S,
                        _ => unreachable!(),
                    };
                    let next = Destination::parse(next);
                    match op {
                        '<' => Self::Lt {
                            category,
                            value,
                            next,
                        },
                        '>' => Self::Gt {
                            category,
                            value,
                            next,
                        },
                        _ => unreachable!(),
                    }
                },
            ),
            map(alpha1, |next| Self::Dest(Destination::parse(next))),
        ))(input)
    }

    fn apply(&self, part: &Part) -> Option<&Destination> {
        match self {
            WorkflowRule::Gt {
                category,
                value,
                next,
            } if part.category_value(category) > *value => Some(next),
            WorkflowRule::Lt {
                category,
                value,
                next,
            } if part.category_value(category) < *value => Some(next),
            WorkflowRule::Dest(next) => Some(next),
            _ => None,
        }
    }

    fn predict(&self, proto: &PartProto) -> (Option<(PartProto, &Destination)>, Option<PartProto>) {
        match self {
            WorkflowRule::Gt {
                category,
                value,
                next,
            } => {
                let (l, r) = proto.split_by_category(category, *value + 1);
                (r.map(|r| (r, next)), l)
            }
            WorkflowRule::Lt {
                category,
                value,
                next,
            } => {
                let (l, r) = proto.split_by_category(category, *value);
                (l.map(|l| (l, next)), r)
            }
            WorkflowRule::Dest(next) => (Some((proto.clone(), next)), None),
        }
    }
}

#[derive(PartialEq)]
enum Destination {
    Accepted,
    Rejected,
    Workflow(String),
}

impl Destination {
    fn parse(s: &str) -> Self {
        match s {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            x => Self::Workflow(x.to_string()),
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<WorkflowRule>,
}

impl Workflow {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            pair(
                alpha1,
                delimited(
                    char('{'),
                    separated_list1(char(','), WorkflowRule::parse),
                    char('}'),
                ),
            ),
            |(name, rules)| Self {
                name: name.to_string(),
                rules,
            },
        )(input)
    }

    fn apply(&self, part: &Part) -> &Destination {
        self.rules.iter().find_map(|r| r.apply(part)).unwrap()
    }

    fn predict(&self, proto: &PartProto) -> Vec<(PartProto, &Destination)> {
        let mut next = Some(proto.clone());
        let mut res = Vec::new();

        let mut rule_index = 0;
        while let Some(n) = next {
            let (c, r) = self.rules[rule_index].predict(&n);
            c.map(|c| res.push(c));
            next = r;
            rule_index += 1;
        }

        res
    }
}

struct WorkflowSet {
    workflows: HashMap<String, Workflow>,
}

impl WorkflowSet {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(line_ending, Workflow::parse), |mut list| {
            Self {
                workflows: HashMap::from_iter(list.drain(0..).map(|w| (w.name.clone(), w))),
            }
        })(input)
    }

    fn apply(&self, part: &Part) -> Destination {
        let mut current_workflow = self.workflows.get("in").unwrap();
        loop {
            match current_workflow.apply(part) {
                Destination::Accepted => return Destination::Accepted,
                Destination::Rejected => return Destination::Rejected,
                Destination::Workflow(next) => current_workflow = self.workflows.get(next).unwrap(),
            }
        }
    }

    fn predict(&self, proto: &PartProto) -> Vec<(PartProto, &Destination)> {
        let mut queue = vec![(proto.clone(), self.workflows.get("in").unwrap())];
        let mut res = Vec::new();
        while let Some((p, w)) = queue.pop() {
            for (p, d) in w.predict(&p) {
                match d {
                    Destination::Accepted => res.push((p, d)),
                    Destination::Rejected => res.push((p, d)),
                    Destination::Workflow(next) => {
                        queue.push((p, self.workflows.get(next).unwrap()))
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}";

        let output = super::part1(input);

        assert_eq!(output, "19114")
    }

    #[test]
    fn test_part2() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}";

        let output = super::part2(input);

        assert_eq!(output, "167409079868000")
    }
}
