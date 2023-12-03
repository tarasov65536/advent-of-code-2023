use std::cmp::{max, min};

fn main() {
    let input = include_str!("day03.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let scheme = Scheme::parse(input);

    scheme.part_numbers().sum::<u32>().to_string()
}

fn part2(input: &str) -> String {
    let scheme = Scheme::parse(input);

    scheme.gears().sum::<u32>().to_string()
}

struct Scheme {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

struct Number {
    line: usize,
    index: usize,
    len: usize,
    value: u32,
}

struct Symbol {
    line: usize,
    index: usize,
    value: char,
}

impl Scheme {
    fn parse(input: &str) -> Self {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        for (j, line) in input.lines().enumerate() {
            let mut left = None;
            for (i, ch) in line.chars().enumerate() {
                if ch.is_ascii_digit() {
                    if left.is_none() {
                        left = Some(i)
                    }
                } else {
                    if let Some(idx) = left {
                        numbers.push(Number {
                            line: j,
                            index: idx,
                            len: i - idx,
                            value: line[idx..i].parse().unwrap(),
                        });
                        left = None
                    }
                    if ch != '.' {
                        symbols.push(Symbol {
                            line: j,
                            index: i,
                            value: ch,
                        })
                    }
                }
            }
            if let Some(idx) = left {
                numbers.push(Number {
                    line: j,
                    index: idx,
                    len: line.len() - idx,
                    value: line[idx..line.len()].parse().unwrap(),
                });
            }
        }
        Self { numbers, symbols }
    }

    fn part_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        self.symbols.iter().flat_map(|s| {
            self.numbers
                .iter()
                .filter(|&n| {
                    max(s.line, n.line) - min(s.line, n.line) < 2
                        && (n.index <= s.index + 1 && s.index <= n.index + n.len)
                })
                .map(|n| n.value)
        })
    }

    fn gears(&self) -> impl Iterator<Item = u32> + '_ {
        self.symbols.iter().filter_map(|s| {
            if s.value != '*' {
                return None;
            }
            let numbers = self
                .numbers
                .iter()
                .filter(|&n| {
                    max(s.line, n.line) - min(s.line, n.line) < 2
                        && n.index <= s.index + 1
                        && s.index <= n.index + n.len
                })
                .map(|n| n.value)
                .collect::<Vec<_>>();
            if numbers.len() != 2 {
                return None;
            }
            Some(numbers[0] * numbers[1])
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let output = super::part1(input);

        assert_eq!(output, "4361")
    }

    #[test]
    fn test_part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let output = super::part2(input);

        assert_eq!(output, "467835")
    }
}
