use nom::branch::alt;
use nom::character::complete::{alpha1, char, u8};
use nom::combinator::{map, success};
use nom::sequence::pair;
use nom::IResult;

fn main() {
    let input = include_str!("day15.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    input
        .split(",")
        .map(|s| hash(s) as usize)
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut boxes = Vec::with_capacity(256);
    for box_number in 0..boxes.capacity() {
        boxes.push(Box::new(box_number));
    }

    for step in input.split(",").map(|s| Step::parse(s).unwrap().1) {
        apply(&step, &mut boxes[step.box_number()]);
    }

    boxes
        .iter()
        .filter(|&b| !b.lens_slots.is_empty())
        .map(|b| b.focusing_power())
        .sum::<usize>()
        .to_string()
}

fn hash(s: &str) -> u8 {
    s.chars()
        .map(|c| c as u32)
        .fold(0, |a, v| (a + v) * 17 % 256) as u8
}

struct Lens {
    label: String,
    focal_length: u8,
}

struct Box {
    box_number: usize,
    lens_slots: Vec<Lens>,
}

impl Box {
    fn new(box_number: usize) -> Self {
        Self {
            box_number,
            lens_slots: Vec::new(),
        }
    }

    fn focusing_power(&self) -> usize {
        self.lens_slots
            .iter()
            .enumerate()
            .map(|(slot_id, lens)| {
                (self.box_number + 1) * (slot_id + 1) * lens.focal_length as usize
            })
            .sum()
    }

    fn remove_lens(&mut self, label: &str) {
        if let Some(idx) = self
            .lens_slots
            .iter()
            .enumerate()
            .find(|(_, lens)| lens.label == label)
            .map(|(idx, _)| idx)
        {
            self.lens_slots.remove(idx);
        }
    }

    fn insert_lens(&mut self, lens: Lens) {
        if let Some(idx) = self
            .lens_slots
            .iter()
            .enumerate()
            .find(|(_, l)| l.label == lens.label)
            .map(|(idx, _)| idx)
        {
            self.lens_slots[idx] = lens;
        } else {
            self.lens_slots.push(lens);
        }
    }
}

fn apply(step: &Step, b: &mut Box) {
    match step.operation {
        Operation::Remove => b.remove_lens(&step.label),
        Operation::Insert(focal_length) => b.insert_lens(Lens {
            label: step.label.clone(),
            focal_length,
        }),
    }
}

enum Operation {
    Remove,
    Insert(u8),
}

struct Step {
    label: String,
    operation: Operation,
}

impl Step {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            pair(
                alpha1::<&str, _>,
                alt((pair(char('-'), success(0)), pair(char('='), u8))),
            ),
            |(l, op)| Self {
                label: l.to_string(),
                operation: match op {
                    ('-', _) => Operation::Remove,
                    ('=', v) => Operation::Insert(v),
                    _ => unreachable!(),
                },
            },
        )(s)
    }

    fn box_number(&self) -> usize {
        hash(&self.label) as usize
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let output = super::part1(input);

        assert_eq!(output, "1320")
    }

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let output = super::part2(input);

        assert_eq!(output, "145")
    }
}
