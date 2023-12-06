use nom::bytes::complete::take_until;
use nom::character::complete::digit1;
use nom::sequence::{pair, preceded};
use nom::IResult;
use nom::{character::complete::space1, multi::separated_list1};

fn main() {
    let input = include_str!("day06.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let times = input
        .lines()
        .nth(0)
        .map(|line| parse_data_line(line).unwrap().1)
        .unwrap()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let distances = input
        .lines()
        .nth(1)
        .map(|line| parse_data_line(line).unwrap().1)
        .unwrap()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| solve(time, distance))
        .fold(1u64, |seed, (a, b)| seed * (b - a))
        .to_string()
}

fn part2(input: &str) -> String {
    let time = input
        .lines()
        .nth(0)
        .map(|line| {
            parse_data_line(line)
                .unwrap()
                .1
                .concat()
                .parse::<u64>()
                .unwrap()
        })
        .unwrap();

    let distance = input
        .lines()
        .nth(1)
        .map(|line| {
            parse_data_line(line)
                .unwrap()
                .1
                .concat()
                .parse::<u64>()
                .unwrap()
        })
        .unwrap();

    let (a, b) = solve(time, distance);
    (b - a).to_string()
}

fn parse_data_line(line: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        pair(take_until(" "), space1),
        separated_list1(space1, digit1),
    )(line)
}

fn solve(time: u64, distance: u64) -> (u64, u64) {
    let b = time as f64;
    let c = distance as f64;

    let d = b * b - 4.0 * c;

    let x1 = (b - d.sqrt()) / 2.0;
    let x2 = (b + d.sqrt()) / 2.0;

    ((x1.floor() + 1.0) as u64, (x2.ceil() - 1.0) as u64 + 1)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let output = super::part1(input);

        assert_eq!(output, "288")
    }

    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let output = super::part2(input);

        assert_eq!(output, "71503")
    }
}
