use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{multi::separated_list1, IResult};

fn main() {
    let input = include_str!("day13.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    parse_input(input)
        .unwrap()
        .1
        .iter()
        .map(|note| {
            note.find_reflection(0).unwrap_or(0) * 100
                + note.transpose().find_reflection(0).unwrap_or(0)
        })
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    parse_input(input)
        .unwrap()
        .1
        .iter()
        .map(|note| {
            note.find_reflection(1).unwrap_or(0) * 100
                + note.transpose().find_reflection(1).unwrap_or(0)
        })
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Note>> {
    separated_list1(pair(line_ending, line_ending), Note::parse)(input)
}

struct Note {
    pattern: Vec<Vec<char>>,
}

impl Note {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(line_ending, many1(alt((char('.'), char('#'))))),
            |pattern| Self { pattern },
        )(input)
    }

    fn calculate_smudge(&self, i: usize, j: usize) -> usize {
        (0..self.pattern[0].len())
            .filter(|k| self.pattern[i][*k] != self.pattern[j][*k])
            .count()
    }

    fn check_horiz_line(&self, i: usize) -> usize {
        let height = self.pattern.len();

        (0..i + 1)
            .rev()
            .zip(i + 1..height)
            .map(|(u, d)| self.calculate_smudge(u, d))
            .sum()
    }

    fn find_reflection(&self, smudge: usize) -> Option<usize> {
        let height = self.pattern.len();

        (0..height - 1)
            .filter(|i| self.check_horiz_line(*i) == smudge)
            .max()
            .map(|c| c + 1)
    }

    fn transpose(&self) -> Self {
        let height = self.pattern[0].len();
        let width = self.pattern.len();
        let mut pattern = vec![vec!['.'; width]; height];
        for i in 0..width {
            for j in 0..height {
                pattern[j][i] = self.pattern[i][j];
            }
        }
        Self { pattern }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let output = super::part1(input);

        assert_eq!(output, "405")
    }

    #[test]
    fn test_part2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let output = super::part2(input);

        assert_eq!(output, "400")
    }
}
