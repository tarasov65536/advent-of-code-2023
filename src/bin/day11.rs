fn main() {
    let input = include_str!("day11.in");

    let part1_out = part1(input);
    let part2_out = part2(input, 1000000);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let mut universe = load_universe(input);
    expand_universe(&mut universe, 2);
    universe
        .iter()
        .enumerate()
        .map(|(i, g)| {
            universe
                .iter()
                .enumerate()
                .filter(|(j, _)| *j > i)
                .map(|(_, g2)| distance(g, g2))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str, expand_size: usize) -> String {
    let mut universe = load_universe(input);
    expand_universe(&mut universe, expand_size);
    universe
        .iter()
        .enumerate()
        .map(|(i, g)| {
            universe
                .iter()
                .enumerate()
                .filter(|(j, _)| *j > i)
                .map(|(_, g2)| distance(g, g2))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

struct Galaxy {
    x: usize,
    y: usize,
}

fn distance(a: &Galaxy, b: &Galaxy) -> usize {
    ((a.x as i128 - b.x as i128).abs() + (a.y as i128 - b.y as i128).abs()) as usize
}

fn load_universe(input: &str) -> Vec<Galaxy> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _)| Galaxy { x, y })
        })
        .collect()
}

fn expand_universe(universe: &mut [Galaxy], expand_size: usize) {
    let mut i = 0;
    while i < universe.iter().map(|g| g.x).max().unwrap() {
        let next = universe
            .iter()
            .filter(|g| i < g.x)
            .map(|g| g.x)
            .min()
            .unwrap();
        let expand_k = (next - i - 1) * (expand_size - 1);
        for g in universe.iter_mut().filter(|g| i < g.x) {
            g.x += expand_k
        }
        i = next + expand_k;
    }
    let mut i = 0;
    while i < universe.iter().map(|g| g.y).max().unwrap() {
        let next = universe
            .iter()
            .filter(|g| i < g.y)
            .map(|g| g.y)
            .min()
            .unwrap();
        let expand_k = (next - i - 1) * (expand_size - 1);
        for g in universe.iter_mut().filter(|g| i < g.y) {
            g.y += expand_k
        }
        i = next + expand_k;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let output = super::part1(input);

        assert_eq!(output, "374")
    }

    #[test]
    fn test_part2_ex1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let output = super::part2(input, 10);

        assert_eq!(output, "1030")
    }

    #[test]
    fn test_part2_ex2() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let output = super::part2(input, 100);

        assert_eq!(output, "8410")
    }
}
