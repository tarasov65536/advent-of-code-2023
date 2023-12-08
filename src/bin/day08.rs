use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, line_ending, space1};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair, separated_pair};
use nom::IResult;

fn main() {
    let input = include_str!("day08.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let (route, map) = parse_input(input);

    route_length(&map, &route, "AAA", |p| p == "ZZZ").to_string()
}

fn part2(input: &str) -> String {
    let (route, map) = parse_input(input);

    map.starting_points()
        .map(|start| route_length(&map, &route, start, |p| p.ends_with("Z")))
        .fold(1, |a, l| lcm(a, l))
        .to_string()
}

fn parse_input(input: &str) -> (Vec<char>, Map) {
    separated_pair(
        many1(alt((char('R'), char('L')))),
        pair(line_ending, line_ending),
        Map::parse,
    )(input)
    .unwrap()
    .1
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn route_length(
    map: &Map,
    route: &[char],
    start: &str,
    mut end: impl FnMut(&str) -> bool,
) -> usize {
    let mut counter = 0;
    let mut current_point = start;
    while !end(current_point) {
        match route[counter % route.len()] {
            'R' => current_point = &map.point(current_point).right,
            'L' => current_point = &map.point(current_point).left,
            _ => unreachable!(),
        }
        counter += 1;
    }
    counter
}

struct Map {
    nodes: HashMap<String, MapPoint>,
}

impl Map {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(
                line_ending,
                separated_pair(
                    take_until(" "),
                    separated_pair(space1, tag("="), space1),
                    delimited(
                        char('('),
                        separated_pair(take_until(","), tag(", "), take_until(")")),
                        char(')'),
                    ),
                ),
            ),
            |a| Self {
                nodes: a
                    .iter()
                    .map(|(name, (left, right))| {
                        (
                            String::from(*name),
                            MapPoint {
                                left: String::from(*left),
                                right: String::from(*right),
                            },
                        )
                    })
                    .collect::<HashMap<String, MapPoint>>(),
            },
        )(input)
    }

    fn point(&self, name: &str) -> &MapPoint {
        self.nodes.get(name).unwrap()
    }

    fn starting_points(&self) -> impl Iterator<Item = &str> {
        self.nodes
            .keys()
            .filter(|&k| k.ends_with("A"))
            .map(|k| k.as_str())
    }
}

struct MapPoint {
    left: String,
    right: String,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1_ex1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let output = super::part1(input);

        assert_eq!(output, "2")
    }

    #[test]
    fn test_part1_ex2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let output = super::part1(input);

        assert_eq!(output, "6")
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let output = super::part2(input);

        assert_eq!(output, "6")
    }
}
