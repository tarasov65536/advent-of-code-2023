use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u32};
use nom::combinator::map;
use nom::multi::many1;
use nom::{character::complete::space1, multi::separated_list1, sequence::separated_pair, IResult};

fn main() {
    let input = include_str!("day12.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| Record::parse(line).unwrap().1.arrangements_count())
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| Record::parse(line).unwrap().1.unfold().arrangements_count())
        .sum::<usize>()
        .to_string()
}

struct Record {
    dots: Vec<char>,
    blocks: Vec<usize>,
}

impl Record {
    fn parse(line: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                many1(alt((char('.'), char('?'), char('#')))),
                space1,
                separated_list1(tag(","), u32),
            ),
            |(dots, blocks)| Self {
                dots,
                blocks: blocks.iter().map(|&x| x as usize).collect::<Vec<_>>(),
            },
        )(line)
    }

    fn unfold(&self) -> Self {
        let dots = [
            &self.dots[..],
            &self.dots[..],
            &self.dots[..],
            &self.dots[..],
            &self.dots[..],
        ]
        .join(&'?');
        let blocks = [
            &self.blocks[..],
            &self.blocks[..],
            &self.blocks[..],
            &self.blocks[..],
            &self.blocks[..],
        ]
        .concat();
        Self { dots, blocks }
    }

    fn arrangements_count(&self) -> usize {
        let mut memory = HashMap::new();
        self.arrangements_count_r(&mut memory, 0, 0, 0)
    }

    fn arrangements_count_r(
        &self,
        memory: &mut HashMap<(usize, usize, usize), usize>,
        i: usize,
        bi: usize,
        current: usize,
    ) -> usize {
        let key = (i, bi, current);
        if let Some(&v) = memory.get(&key) {
            return v;
        }
        if i == self.dots.len() {
            if bi == self.blocks.len() && current == 0
                || bi == self.blocks.len() - 1 && self.blocks[bi] == current
            {
                return 1;
            } else {
                return 0;
            }
        }

        let answer = ['.', '#']
            .iter()
            .filter(|&c| self.dots[i] == *c || self.dots[i] == '?')
            .map(|&c| {
                if c == '.' && current == 0 {
                    self.arrangements_count_r(memory, i + 1, bi, 0)
                } else if c == '.'
                    && current > 0
                    && bi < self.blocks.len()
                    && self.blocks[bi] == current
                {
                    self.arrangements_count_r(memory, i + 1, bi + 1, 0)
                } else if c == '#' {
                    self.arrangements_count_r(memory, i + 1, bi, current + 1)
                } else {
                    0
                }
            })
            .sum::<usize>();

        memory.insert(key, answer);
        answer
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let output = super::part1(input);

        assert_eq!(output, "21")
    }

    #[test]
    fn test_part2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let output = super::part2(input);

        assert_eq!(output, "525152")
    }
}
