use nom::branch::alt;
use nom::character::complete::{space1, u32};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, IResult};

fn main() {
    let input = include_str!("day02.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .filter(|(_, variants)| {
            variants.iter().all(|cs| match cs {
                Cubes::Red(n) if *n <= 12 => true,
                Cubes::Green(n) if *n <= 13 => true,
                Cubes::Blue(n) if *n <= 14 => true,
                _ => false,
            })
        })
        .map(|(id, _)| id)
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .map(|(_, variants)| {
            let red = variants
                .iter()
                .filter_map(|x| if let Cubes::Red(n) = x { Some(n) } else { None })
                .max()
                .unwrap_or(&0);
            let green = variants
                .iter()
                .filter_map(|x| {
                    if let Cubes::Green(n) = x {
                        Some(n)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(&0);
            let blue = variants
                .iter()
                .filter_map(|x| {
                    if let Cubes::Blue(n) = x {
                        Some(n)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(&0);
            red * green * blue
        })
        .sum::<u32>()
        .to_string()
}

fn parse_game(input: &str) -> IResult<&str, (u32, Vec<Cubes>)> {
    separated_pair(
        map(separated_pair(tag("Game"), space1, u32), |(_, id)| id),
        tag(": "),
        map(
            separated_list0(
                tag("; "),
                separated_list0(
                    tag(", "),
                    map(
                        separated_pair(u32, space1, alt((tag("red"), tag("green"), tag("blue")))),
                        |(cnt, clr)| match clr {
                            "red" => Cubes::Red(cnt),
                            "green" => Cubes::Green(cnt),
                            "blue" => Cubes::Blue(cnt),
                            _ => unreachable!(),
                        },
                    ),
                ),
            ),
            |x| {
                x.iter()
                    .flat_map(|x| x.iter().map(|c| *c))
                    .collect::<Vec<_>>()
            },
        ),
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cubes {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[cfg(test)]
mod tests {
    use crate::Cubes;

    #[test]
    fn test_parse_game() {
        let input = "Game 15: 1 blue, 2 red; 3 red, 4 green, 5 blue; 6 green";

        assert_eq!(
            super::parse_game(input),
            Ok((
                "",
                (
                    15,
                    vec![
                        Cubes::Blue(1),
                        Cubes::Red(2),
                        Cubes::Red(3),
                        Cubes::Green(4),
                        Cubes::Blue(5),
                        Cubes::Green(6)
                    ]
                )
            ))
        )
    }

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let output = super::part1(input);

        assert_eq!(output, "8")
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let output = super::part2(input);

        assert_eq!(output, "2286")
    }
}
