use nom::character::complete::{line_ending, not_line_ending, space1, u128};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{pair, separated_pair, tuple};
use nom::{bytes::complete::tag, IResult};

fn main() {
    let input = include_str!("day05.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let almanac = Almanac::parse(input).unwrap().1;

    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.location(seed))
        .min()
        .unwrap()
        .to_string()
}

fn part2(input: &str) -> String {
    let almanac = Almanac::parse(input).unwrap().1;

    almanac
        .seeds
        .chunks(2)
        .map(|r| (r[0], r[1]))
        .flat_map(|(start, length)| almanac.locations(start, length))
        .map(|r| r.0)
        .min()
        .unwrap()
        .to_string()
}

struct Almanac {
    seeds: Vec<u128>,
    maps: Vec<Map>,
}

impl Almanac {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                tuple((tag("seeds: "), separated_list1(space1, u128))),
                pair(line_ending, line_ending),
                separated_list1(pair(line_ending, line_ending), Map::parse),
            ),
            |((_, seeds), maps)| Self { seeds, maps },
        )(input)
    }

    fn location(&self, seed: u128) -> u128 {
        self.maps.iter().fold(seed, |seed, m| m.convert(seed))
    }

    fn locations(&self, seed: u128, length: u128) -> Vec<(u128, u128)> {
        let mut converted = vec![(seed, length)];
        let mut tmp = vec![];
        for map in self.maps.iter() {
            for (start, length) in converted.iter() {
                let mut conv = map.convert_range(*start, *length);
                tmp.append(&mut conv);
            }
            converted.clear();
            converted.append(&mut tmp);
        }
        converted
    }
}

struct Map {
    converters: Vec<RangeConverter>,
}

impl Map {
    fn parse(block: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                not_line_ending,
                line_ending,
                separated_list1(line_ending, RangeConverter::parse),
            ),
            |(_, converters)| Self { converters },
        )(block)
    }

    fn convert(&self, seed: u128) -> u128 {
        self.converters
            .iter()
            .find_map(|c| c.convert(seed))
            .unwrap_or(seed)
    }

    fn convert_range(&self, start: u128, length: u128) -> Vec<(u128, u128)> {
        let mut converted = vec![];
        let mut pending = vec![(start, length)];
        let mut tmp = vec![];
        for converter in &self.converters {
            for r in &pending {
                let (mut c, mut p) = converter.convert_range(r.0, r.1);
                converted.append(&mut c);
                tmp.append(&mut p);
            }
            pending.clear();
            pending.append(&mut tmp);
        }
        converted.append(&mut pending);
        converted
    }
}

struct RangeConverter {
    dst_start: u128,
    src_start: u128,
    length: u128,
}

impl RangeConverter {
    fn parse(line: &str) -> IResult<&str, Self> {
        map(
            tuple((u128, space1, u128, space1, u128)),
            |(dst_start, _, src_start, _, length)| Self {
                dst_start,
                src_start,
                length,
            },
        )(line)
    }

    fn convert(&self, src: u128) -> Option<u128> {
        if src < self.src_start || self.src_start + self.length < src {
            None
        } else {
            let diff = src - self.src_start;
            Some(self.dst_start + diff)
        }
    }

    fn convert_range(&self, seed: u128, length: u128) -> (Vec<(u128, u128)>, Vec<(u128, u128)>) {
        let seed_end = seed + length;
        let range_end = self.src_start + self.length;
        let range_dst_end = self.dst_start + self.length;
        match (self.convert(seed), self.convert(seed + length)) {
            (Some(dst), Some(_)) => (vec![(dst, length)], vec![]),
            (None, None) if seed < self.src_start && range_end < seed_end => (
                vec![(self.dst_start, self.length)],
                vec![
                    (seed, self.src_start - seed),
                    (range_end, seed_end - range_end),
                ],
            ),
            (None, None) => (vec![], vec![(seed, length)]),
            (None, Some(dst_end)) => (
                vec![(self.dst_start, dst_end - self.dst_start)],
                vec![(seed, self.src_start - seed)],
            ),
            (Some(dst_start), None) => (
                vec![(dst_start, range_dst_end - dst_start)],
                vec![(range_end, seed_end - range_end)],
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let output = super::part1(input);

        assert_eq!(output, "35")
    }

    #[test]
    fn test_part2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let output = super::part2(input);

        assert_eq!(output, "46")
    }
}
