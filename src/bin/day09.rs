use nom::character::complete::{i64, line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;

fn main() {
    let input = include_str!("day09.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let inputs = parse_input(input).unwrap().1;

    inputs
        .iter()
        .map(|seq| predict(&seq))
        .sum::<i64>()
        .to_string()
}

fn part2(input: &str) -> String {
    let inputs = parse_input(input).unwrap().1;

    inputs
        .iter()
        .map(|seq| predict_back(&seq))
        .sum::<i64>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, separated_list1(space1, i64))(input)
}

fn derive(seq: &[i64]) -> Vec<i64> {
    seq.iter()
        .zip(seq[1..].iter())
        .map(|(a, b)| b - a)
        .collect()
}

fn predict(seq: &[i64]) -> i64 {
    if seq.iter().all(|&x| x == 0) {
        return 0;
    }

    let d = derive(seq);
    let n = predict(&d);

    seq.last().unwrap() + n
}

fn predict_back(seq: &[i64]) -> i64 {
    if seq.iter().all(|&x| x == 0) {
        return 0;
    }

    let d = derive(seq);
    let n = predict_back(&d);

    seq.first().unwrap() - n
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let output = super::part1(input);

        assert_eq!(output, "114")
    }

    #[test]
    fn test_part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let output = super::part2(input);

        assert_eq!(output, "2")
    }
}
