use std::collections::HashMap;

use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::map;
use nom::multi::many1;
use nom::{multi::separated_list1, IResult};

fn main() {
    let input = include_str!("day14.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    Platform::parse(input)
        .unwrap()
        .1
        .roll()
        .total_load()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut memory = HashMap::new();
    let mut platform = Platform::parse(input).unwrap().1;
    let mut t = 0;
    let target = 1000000000;
    while t < target {
        t += 1;
        for _ in 0..4 {
            platform.roll();
            platform.rotate();
        }
        let key = platform.tiles.iter().flatten().collect::<String>();
        if let Some(x) = memory.get(&key) {
            let cl = t - x;
            t += (target - t) / cl * cl
        }
        memory.insert(key, t);
    }
    platform.total_load().to_string()
}

struct Platform {
    tiles: Vec<Vec<char>>,
}

impl Platform {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(line_ending, many1(alt((char('.'), char('O'), char('#'))))),
            |tiles| Self { tiles },
        )(input)
    }

    fn roll(&mut self) -> &Self {
        let width = self.tiles[0].len();
        let height = self.tiles.len();

        for c in 0..width {
            for _ in 0..height {
                for r in 0..height {
                    if self.tiles[r][c] == 'O' && r > 0 && self.tiles[r - 1][c] == '.' {
                        self.tiles[r - 1][c] = 'O';
                        self.tiles[r][c] = '.';
                    }
                }
            }
        }
        self
    }

    fn rotate(&mut self) -> &Self {
        let width = self.tiles[0].len();
        let height = self.tiles.len();
        let mut new_tiles = self.tiles.clone();
        for r in 0..height {
            for c in 0..width {
                new_tiles[c][height - 1 - r] = self.tiles[r][c]
            }
        }
        self.tiles = new_tiles;
        self
    }

    fn total_load(&self) -> usize {
        let width = self.tiles[0].len();
        let height = self.tiles.len();
        (0..width)
            .map(|x| {
                (0..height)
                    .filter(|y| self.tiles[*y][x] == 'O')
                    .map(|y| height - y)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let output = super::part1(input);

        assert_eq!(output, "136")
    }

    #[test]
    fn test_part2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let output = super::part2(input);

        assert_eq!(output, "64")
    }
}
