fn main() {
    let input = include_str!("day01.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|x| {
            x.chars()
                .find(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .unwrap()
                * 10
                + x.chars()
                    .rfind(|c| c.is_ascii_digit())
                    .map(|c| c.to_digit(10).unwrap())
                    .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

struct Digit {
    code: &'static str,
    value: u32,
}

static DIGITS: [Digit; 18] = [
    Digit {
        code: "one",
        value: 1,
    },
    Digit {
        code: "two",
        value: 2,
    },
    Digit {
        code: "three",
        value: 3,
    },
    Digit {
        code: "four",
        value: 4,
    },
    Digit {
        code: "five",
        value: 5,
    },
    Digit {
        code: "six",
        value: 6,
    },
    Digit {
        code: "seven",
        value: 7,
    },
    Digit {
        code: "eight",
        value: 8,
    },
    Digit {
        code: "nine",
        value: 9,
    },
    Digit {
        code: "1",
        value: 1,
    },
    Digit {
        code: "2",
        value: 2,
    },
    Digit {
        code: "3",
        value: 3,
    },
    Digit {
        code: "4",
        value: 4,
    },
    Digit {
        code: "5",
        value: 5,
    },
    Digit {
        code: "6",
        value: 6,
    },
    Digit {
        code: "7",
        value: 7,
    },
    Digit {
        code: "8",
        value: 8,
    },
    Digit {
        code: "9",
        value: 9,
    },
];

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let digits = DIGITS
                .iter()
                .flat_map(|d| line.match_indices(d.code).map(|(i, _)| (i, d.value)))
                .collect::<Vec<_>>();
            digits.iter().min_by_key(|d| d.0).map(|d| d.1).unwrap() * 10
                + digits.iter().max_by_key(|d| d.0).map(|d| d.1).unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let output = super::part1(input);

        assert_eq!(output, "142")
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let output = super::part2(input);

        assert_eq!(output, "281")
    }
}
