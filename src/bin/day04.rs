use std::cmp::min;

use nom::character::complete::{space1, u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::{bytes::complete::tag, IResult};

fn main() {
    let input = include_str!("day04.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| parse_card(line).unwrap().1.points())
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let cards = input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect::<Vec<_>>();
    let mut card_counters = vec![1usize; cards.len()];
    for card in cards.iter() {
        let copies = card.match_count();
        let generated = card_counters[card.id - 1];
        let min_copy_idx = card.id;
        let max_copy_idx = min(cards.len(), card.id + copies);
        for counter in &mut card_counters[min_copy_idx..max_copy_idx] {
            *counter += generated;
        }
    }

    card_counters.iter().sum::<usize>().to_string()
}

struct Card {
    id: usize,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn match_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }

    fn points(&self) -> u32 {
        let matches = self.match_count() as u32;
        if matches == 0 {
            0
        } else {
            (2u32).pow(matches - 1)
        }
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map(
        separated_pair(
            separated_pair(tag("Card"), space1, u32),
            tuple((tag(":"), space1)),
            separated_pair(
                separated_list1(space1, u32),
                delimited(space1, tag("|"), space1),
                separated_list1(space1, u32),
            ),
        ),
        |((_, id), (winning_numbers, numbers))| Card {
            id: id as usize,
            winning_numbers,
            numbers,
        },
    )(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let output = super::part1(input);

        assert_eq!(output, "13")
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let output = super::part2(input);

        assert_eq!(output, "30")
    }
}
