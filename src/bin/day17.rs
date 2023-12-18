use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    let input = include_str!("day17.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    HeatMap::parse(input).minimum_heat_loss().to_string()
}

fn part2(input: &str) -> String {
    HeatMap::parse(input).minimum_heat_loss_ultra().to_string()
}

struct HeatMap {
    tiles: Vec<Vec<u32>>,
}

impl HeatMap {
    fn parse(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { tiles }
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn neightbours(&self, pos: Position) -> impl Iterator<Item = (Position, Direction)> {
        [
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Up,
        ]
        .iter()
        .filter_map(move |&dir| self.step(pos, dir).map(|pos| (pos, dir)))
        .collect::<Vec<_>>()
        .into_iter()
    }

    fn step(&self, pos: Position, dir: Direction) -> Option<Position> {
        match dir {
            Direction::Up if pos.y > 0 => Some(Position::new(pos.x, pos.y - 1)),
            Direction::Down if pos.y < self.height() - 1 => Some(Position::new(pos.x, pos.y + 1)),
            Direction::Left if pos.x > 0 => Some(Position::new(pos.x - 1, pos.y)),
            Direction::Right if pos.x < self.width() - 1 => Some(Position::new(pos.x + 1, pos.y)),
            _ => None,
        }
    }

    fn heat(&self, pos: Position) -> u32 {
        self.tiles[pos.y][pos.x]
    }

    fn minimum_heat_loss_ultra(&self) -> u32 {
        let mut distances = HashMap::new();
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, Position::new(0, 0), Direction::Right, 0)));
        while let Some(Reverse((dist, pos, dir, sdir))) = queue.pop() {
            let key = (pos, dir, sdir);
            if distances.contains_key(&key) {
                continue;
            }
            distances.insert(key, dist);
            for (npos, ndir) in self.neightbours(pos).filter(|(_, d)| d.opposite() != dir) {
                let nsdir = if ndir != dir { 1 } else { sdir + 1 };

                if nsdir <= 10 && (dir == ndir || sdir >= 4 || sdir == 0) {
                    let cost = self.heat(npos);
                    queue.push(Reverse((dist + cost, npos, ndir, nsdir)));
                }
            }
        }

        distances
            .iter()
            .filter(|((pos, _, sdir), _)| {
                pos.x == self.width() - 1 && pos.y == self.height() - 1 && *sdir >= 4
            })
            .map(|(_, d)| *d)
            .min()
            .unwrap()
    }

    fn minimum_heat_loss(&self) -> u32 {
        let mut distances = HashMap::new();
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, Position::new(0, 0), Direction::Right, 0)));
        while let Some(Reverse((dist, pos, dir, sdir))) = queue.pop() {
            let key = (pos, dir, sdir);
            if distances.contains_key(&key) {
                continue;
            }
            distances.insert(key, dist);
            for (npos, ndir) in self.neightbours(pos).filter(|(_, d)| d.opposite() != dir) {
                let nsdir = if ndir != dir { 1 } else { sdir + 1 };

                if nsdir <= 3 {
                    let cost = self.heat(npos);
                    queue.push(Reverse((dist + cost, npos, ndir, nsdir)));
                }
            }
        }

        distances
            .iter()
            .filter(|((pos, _, _), _)| pos.x == self.width() - 1 && pos.y == self.height() - 1)
            .map(|(_, d)| *d)
            .min()
            .unwrap()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let output = super::part1(input);

        assert_eq!(output, "102")
    }

    #[test]
    fn test_part2_ex1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let output = super::part2(input);

        assert_eq!(output, "94")
    }

    #[test]
    fn test_part2_ex2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";

        let output = super::part2(input);

        assert_eq!(output, "71")
    }
}
