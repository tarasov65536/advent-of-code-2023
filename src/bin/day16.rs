use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("day16.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let map = Map::parse(input);

    let beam_map = map.trace_beam(Position::new(0, 0), Direction::Right);

    beam_map.energized_tile_count().to_string()
}

fn part2(input: &str) -> String {
    let map = Map::parse(input);

    (0..map.width)
        .flat_map(|x| {
            [
                (Position::new(x, 0), Direction::Down),
                (Position::new(x, map.heigth - 1), Direction::Up),
            ]
        })
        .chain((0..map.heigth).flat_map(|y| {
            [
                (Position::new(0, y), Direction::Right),
                (Position::new(map.width - 1, y), Direction::Left),
            ]
        }))
        .map(|(start, dir)| map.trace_beam(start, dir).energized_tile_count())
        .max()
        .unwrap()
        .to_string()
}

struct Map {
    tiles: HashMap<Position, char>,
    width: usize,
    heigth: usize,
}

impl Map {
    fn parse(input: &str) -> Self {
        let cells = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self {
            width: cells[0].len(),
            heigth: cells.len(),
            tiles: cells
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter_map(move |(x, ch)| (*ch != '.').then(|| (Position::new(x, y), *ch)))
                })
                .collect(),
        }
    }

    fn trace_beam(&self, start: Position, dir: Direction) -> BeamMap {
        let mut beam_map = BeamMap {
            tiles: HashSet::new(),
        };

        let mut next = vec![Step::new(start, dir)];
        while let Some(step) = next.pop() {
            if !beam_map.tiles.insert(step) {
                continue;
            }
            match (step.dir, self.tiles.get(&step.pos)) {
                (x, Some('|')) if x == Direction::Left || x == Direction::Right => {
                    if let Some(np) = self.next_position(step.pos, Direction::Up) {
                        next.push(Step::new(np, Direction::Up));
                    }
                    if let Some(np) = self.next_position(step.pos, Direction::Down) {
                        next.push(Step::new(np, Direction::Down));
                    }
                }
                (x, Some('-')) if x == Direction::Up || x == Direction::Down => {
                    if let Some(np) = self.next_position(step.pos, Direction::Left) {
                        next.push(Step::new(np, Direction::Left));
                    }
                    if let Some(np) = self.next_position(step.pos, Direction::Right) {
                        next.push(Step::new(np, Direction::Right));
                    }
                }
                (Direction::Right, Some('\\')) => {
                    if let Some(np) = self.next_position(step.pos, Direction::Down) {
                        next.push(Step::new(np, Direction::Down));
                    }
                }
                (Direction::Left, Some('\\')) => {
                    if let Some(np) = self.next_position(step.pos, Direction::Up) {
                        next.push(Step::new(np, Direction::Up));
                    }
                }
                (Direction::Down, Some('\\')) => {
                    if let Some(np) = self.next_position(step.pos, Direction::Right) {
                        next.push(Step::new(np, Direction::Right));
                    }
                }
                (Direction::Up, Some('\\')) => {
                    if let Some(np) = self.next_position(step.pos, Direction::Left) {
                        next.push(Step::new(np, Direction::Left));
                    }
                }
                (Direction::Right, Some('/')) => {
                    if let Some(np) = self.next_position(step.pos, Direction::Up) {
                        next.push(Step::new(np, Direction::Up));
                    }
                }
                (Direction::Left, Some('/')) => {
                    if let Some(np) = self.next_position(step.pos, Direction::Down) {
                        next.push(Step::new(np, Direction::Down));
                    }
                }
                (Direction::Down, Some('/')) => {
                    if let Some(np) = self.next_position(step.pos, Direction::Left) {
                        next.push(Step::new(np, Direction::Left));
                    }
                }
                (Direction::Up, Some('/')) => {
                    if let Some(np) = self.next_position(step.pos, Direction::Right) {
                        next.push(Step::new(np, Direction::Right));
                    }
                }
                (dir, _) => {
                    if let Some(np) = self.next_position(step.pos, dir) {
                        next.push(Step::new(np, dir));
                    }
                }
            };
        }

        beam_map
    }

    fn next_position(&self, cur: Position, dir: Direction) -> Option<Position> {
        match dir {
            Direction::Up if cur.y() > 0 => Some(Position::new(cur.x(), cur.y() - 1)),
            Direction::Down if cur.y() < self.heigth - 1 => {
                Some(Position::new(cur.x(), cur.y() + 1))
            }
            Direction::Left if cur.x() > 0 => Some(Position::new(cur.x() - 1, cur.y())),
            Direction::Right if cur.x() < self.width - 1 => {
                Some(Position::new(cur.x() + 1, cur.y()))
            }
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position((usize, usize));

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self((x, y))
    }

    fn x(&self) -> usize {
        self.0 .0
    }

    fn y(&self) -> usize {
        self.0 .1
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Step {
    pos: Position,
    dir: Direction,
}

impl Step {
    fn new(pos: Position, dir: Direction) -> Self {
        Self { pos, dir }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct BeamMap {
    tiles: HashSet<Step>,
}

impl BeamMap {
    fn energized_tile_count(&self) -> usize {
        self.tiles
            .iter()
            .map(|t| t.pos)
            .collect::<HashSet<_>>()
            .len()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

        let output = super::part1(input);

        assert_eq!(output, "46")
    }

    #[test]
    fn test_part2() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

        let output = super::part2(input);

        assert_eq!(output, "51")
    }
}
