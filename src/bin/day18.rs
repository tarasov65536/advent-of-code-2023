use nom::{
    branch::alt,
    character::complete::u128,
    character::complete::{char, hex_digit1, space1},
    combinator::map,
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn main() {
    let input = include_str!("day18.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let steps = parse_input(input);
    let p = steps.iter().map(|s| s.length).sum::<u128>();
    let a = area(&steps);

    (a + p / 2 + 1).to_string()
}

fn part2(input: &str) -> String {
    let steps = parse_input(input)
        .iter()
        .map(|x| x.decode())
        .collect::<Vec<_>>();
    let p = steps.iter().map(|s| s.length).sum::<u128>();
    let a = area(&steps);

    (a + p / 2 + 1).to_string()
}

fn area(steps: &[DigStep]) -> u128 {
    let mut a = 0i128;
    let mut y = 0i128;
    for step in steps {
        match step.direction {
            Direction::Right => a += y * step.length as i128,
            Direction::Left => a -= y * step.length as i128,
            Direction::Down => y -= step.length as i128,
            Direction::Up => y += step.length as i128,
        }
    }
    a as u128
}

fn parse_input(input: &str) -> Vec<DigStep> {
    input
        .lines()
        .map(|line| DigStep::parse(line).unwrap().1)
        .collect()
}

#[derive(Debug)]
struct DigStep {
    direction: Direction,
    length: u128,
    color: String,
}

impl DigStep {
    fn decode(&self) -> Self {
        let len = u128::from_str_radix(&self.color[0..self.color.len() - 1], 16).unwrap();
        let dir = match self.color.chars().last().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => unreachable!(),
        };

        Self {
            direction: dir,
            length: len,
            color: self.color.clone(),
        }
    }
}

fn color(input: &str) -> IResult<&str, &str> {
    preceded(char('#'), hex_digit1)(input)
}

impl DigStep {
    fn parse(line: &str) -> IResult<&str, Self> {
        map(
            tuple((
                Direction::parse,
                delimited(space1, u128, space1),
                delimited(char('('), color, char(')')),
            )),
            |(direction, length, color)| Self {
                direction,
                length,
                color: color.to_string(),
            },
        )(line)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            alt((char('R'), char('L'), char('U'), char('D'))),
            |c| match c {
                'R' => Self::Right,
                'L' => Self::Left,
                'U' => Self::Up,
                'D' => Self::Down,
                _ => unreachable!(),
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let output = super::part1(input);

        assert_eq!(output, "62")
    }

    #[test]
    fn test_part2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let output = super::part2(input);

        assert_eq!(output, "952408144115")
    }
}
