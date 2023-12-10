fn main() {
    let input = include_str!("day10.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (find_longest_loop(&map).len() / 2).to_string()
}

fn part2(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let size = (map[0].len(), map.len());

    let longest_loop = find_longest_loop(&map);

    let mut map = vec![vec!['.'; size.0]; size.1];
    for (x, y, ch) in &longest_loop {
        map[*y][*x] = *ch;
    }

    map.iter()
        .map(|line| score_line(line))
        .sum::<usize>()
        .to_string()
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn find_longest_loop(map: &Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
    let (sx, sy) = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, &ch)| (x, y, ch)))
        .find(|(_, _, ch)| *ch == 'S')
        .map(|(x, y, _)| (x, y))
        .unwrap();

    [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .iter()
    .filter_map(|&initial_dir| {
        let mut path = vec![];
        let mut cx = sx;
        let mut cy = sy;
        let mut dir = initial_dir;
        while let Some((nx, ny, ch)) = step(&map, cx, cy, dir) {
            if ch == 'S' {
                path.push((nx, ny, start_subs(initial_dir, dir)));
                return Some(path);
            }
            path.push((nx, ny, ch));
            dir = match (dir, ch) {
                (Direction::North, '|') => Direction::North,
                (Direction::South, '|') => Direction::South,
                (Direction::North, '7') => Direction::West,
                (Direction::North, 'F') => Direction::East,
                (Direction::South, 'J') => Direction::West,
                (Direction::South, 'L') => Direction::East,
                (Direction::East, '-') => Direction::East,
                (Direction::West, '-') => Direction::West,
                (Direction::East, '7') => Direction::South,
                (Direction::East, 'J') => Direction::North,
                (Direction::West, 'F') => Direction::South,
                (Direction::West, 'L') => Direction::North,
                _ => unreachable!(),
            };
            cx = nx;
            cy = ny;
        }
        None
    })
    .last()
    .unwrap()
}

fn start_subs(initial_dir: Direction, final_dir: Direction) -> char {
    match (initial_dir, final_dir) {
        (Direction::North, Direction::North) | (Direction::South, Direction::South) => '|',
        (Direction::East, Direction::East) | (Direction::West, Direction::West) => '-',
        (Direction::North, Direction::East) => 'J',
        (Direction::North, Direction::West) => 'L',
        (Direction::South, Direction::East) => '7',
        (Direction::South, Direction::West) => 'F',

        (Direction::East, Direction::South) => 'L',
        (Direction::West, Direction::South) => 'J',
        (Direction::East, Direction::North) => 'F',
        (Direction::West, Direction::North) => '7',
        _ => unreachable!(),
    }
}

fn score_line(line: &[char]) -> usize {
    let mut score = 0;
    let mut crossings = 0;
    for c in line {
        match c {
            '.' => {
                if crossings % 2 != 0 {
                    score += 1;
                }
            }
            '|' | 'F' | '7' => {
                crossings += 1;
            }
            _ => {}
        }
    }
    score
}

fn step(
    map: &Vec<Vec<char>>,
    sx: usize,
    sy: usize,
    dir: Direction,
) -> Option<(usize, usize, char)> {
    match dir {
        Direction::North if sy == 0 => return None,
        Direction::South if sy == map.len() - 1 => return None,
        Direction::East if sx == map.first().unwrap().len() - 1 => return None,
        Direction::West if sx == 0 => return None,
        _ => {}
    };

    let (nx, ny) = match (map[sy][sx], dir) {
        ('|', Direction::North)
        | ('S', Direction::North)
        | ('L', Direction::North)
        | ('J', Direction::North) => (sx, sy - 1),
        ('|', Direction::South)
        | ('S', Direction::South)
        | ('7', Direction::South)
        | ('F', Direction::South) => (sx, sy + 1),
        ('-', Direction::East)
        | ('S', Direction::East)
        | ('L', Direction::East)
        | ('F', Direction::East) => (sx + 1, sy),
        ('-', Direction::West)
        | ('S', Direction::West)
        | ('J', Direction::West)
        | ('7', Direction::West) => (sx - 1, sy),
        _ => return None,
    };

    match (dir, map[ny][nx]) {
        (Direction::North, '|')
        | (Direction::North, 'S')
        | (Direction::North, '7')
        | (Direction::North, 'F') => Some((nx, ny, map[ny][nx])),
        (Direction::South, '|')
        | (Direction::South, 'S')
        | (Direction::South, 'J')
        | (Direction::South, 'L') => Some((nx, ny, map[ny][nx])),
        (Direction::East, '-')
        | (Direction::East, 'S')
        | (Direction::East, 'J')
        | (Direction::East, '7') => Some((nx, ny, map[ny][nx])),
        (Direction::West, '-')
        | (Direction::West, 'S')
        | (Direction::West, 'L')
        | (Direction::West, 'F') => Some((nx, ny, map[ny][nx])),
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1_ex1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let output = super::part1(input);

        assert_eq!(output, "4")
    }

    #[test]
    fn test_part1_ex2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let output = super::part1(input);

        assert_eq!(output, "8")
    }

    #[test]
    fn test_part2_ex1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let output = super::part2(input);

        assert_eq!(output, "4")
    }

    #[test]
    fn test_part2_ex2() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        let output = super::part2(input);

        assert_eq!(output, "4")
    }

    #[test]
    fn test_part2_ex3() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let output = super::part2(input);

        assert_eq!(output, "8")
    }

    #[test]
    fn test_part2_ex4() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let output = super::part2(input);

        assert_eq!(output, "10")
    }
}
