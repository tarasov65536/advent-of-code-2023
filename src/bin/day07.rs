use std::collections::HashMap;

use nom::character::complete::{anychar, space1, u32};
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::{sequence::tuple, IResult};

fn main() {
    let input = include_str!("day07.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let mut records = input
        .lines()
        .map(|line| Record::parse(line).unwrap().1)
        .collect::<Vec<_>>();

    records.sort_by(|a, b| a.hand.cmp(&b.hand));

    records
        .iter()
        .enumerate()
        .map(|(i, r)| r.bid * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut records = input
        .lines()
        .map(|line| Record::parse(line).unwrap().1)
        .collect::<Vec<_>>();

    for record in records.iter_mut() {
        for card in record.hand.0.iter_mut() {
            if card == &Card::Jack {
                *card = Card::Joker;
            }
        }
    }

    records.sort_by(|a, b| a.hand.cmp(&b.hand));

    records
        .iter()
        .enumerate()
        .map(|(i, r)| r.bid * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err("invalid value"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
struct Hand([Card; 5]);

impl Hand {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((anychar, anychar, anychar, anychar, anychar)),
            |(a, b, c, d, e)| {
                Self([
                    a.try_into().unwrap(),
                    b.try_into().unwrap(),
                    c.try_into().unwrap(),
                    d.try_into().unwrap(),
                    e.try_into().unwrap(),
                ])
            },
        )(input)
    }

    fn kind(&self) -> HandKind {
        let mut counter: HashMap<Card, usize> = HashMap::new();
        for card in self.0.iter() {
            *counter.entry(*card).or_default() += 1;
        }
        let jokers = *counter.get(&Card::Joker).unwrap_or(&0);

        if counter.len() == 1 {
            HandKind::FiveOfAKind
        } else if counter.len() == 2 {
            let m = *counter.values().max().unwrap();
            match (m, jokers) {
                (4, 0) => HandKind::FourOfAKind,
                (3, 0) => HandKind::FullHouse,
                (_, _) => HandKind::FiveOfAKind,
            }
        } else if *counter.values().max().unwrap() == 3 {
            if jokers != 0 {
                HandKind::FourOfAKind
            } else {
                HandKind::ThreeOfAKind
            }
        } else if counter.values().filter(|&&v| v == 2).count() == 2 {
            if jokers == 2 {
                HandKind::FourOfAKind
            } else if jokers == 1 {
                HandKind::FullHouse
            } else {
                HandKind::TwoPair
            }
        } else if *counter.values().max().unwrap() == 2 {
            if jokers == 2 {
                HandKind::ThreeOfAKind
            } else if jokers == 1 {
                HandKind::ThreeOfAKind
            } else {
                HandKind::OnePair
            }
        } else if jokers == 1 {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind()
            .cmp(&other.kind())
            .then_with(|| self.0.partial_cmp(&other.0).unwrap())
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Record {
    hand: Hand,
    bid: u32,
}

impl Record {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_pair(Hand::parse, space1, u32), |(hand, bid)| {
            Self { hand, bid }
        })(input)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let output = super::part1(input);

        assert_eq!(output, "6440")
    }

    #[test]
    fn test_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let output = super::part2(input);

        assert_eq!(output, "5905")
    }
}
