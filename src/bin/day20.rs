use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    character::complete::char,
    combinator::map,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("day20.in");

    let part1_out = part1(input);
    let part2_out = part2(input);

    println!("Part1: {}", part1_out);
    println!("Part2: {}", part2_out);
}

fn part1(input: &str) -> String {
    let mut network = input
        .lines()
        .map(|line| parse_input(line).unwrap().1)
        .collect::<Vec<_>>();
    for i in 0..network.len() {
        let name = network[i].name;
        network[i].inputs = network
            .iter_mut()
            .filter_map(|d| d.outputs.contains(&name).then_some(d.name))
            .collect();
    }

    let mut modules = network
        .iter()
        .map(|d| Module::from_desscriptor(d))
        .collect::<Vec<_>>();

    let mut processed = Vec::new();

    for _ in 0..1000 {
        let mut outputs: VecDeque<_> = [Pulse::new(
            "button".into(),
            "broadcaster".into(),
            PulseValue::Low,
        )]
        .into();
        while let Some(pulse) = outputs.pop_front() {
            if let Some(module) = modules.iter_mut().find(|m| m.name == pulse.dst) {
                module.process(&pulse, &mut outputs);
            }
            processed.push(pulse);
        }
    }

    let (lo, hi) = processed.iter().fold((0, 0), |(lo, hi), x| match x.value {
        PulseValue::High => (lo, hi + 1),
        PulseValue::Low => (lo + 1, hi),
    });

    (lo * hi).to_string()
}

fn part2(input: &str) -> String {
    let mut network = input
        .lines()
        .map(|line| parse_input(line).unwrap().1)
        .collect::<Vec<_>>();
    for i in 0..network.len() {
        let name = network[i].name;
        network[i].inputs = network
            .iter_mut()
            .filter_map(|d| d.outputs.contains(&name).then_some(d.name))
            .collect();
    }

    let mut modules = network
        .iter()
        .map(|d| Module::from_desscriptor(d))
        .collect::<Vec<_>>();

    //3739 4001 3943 3821

    let mut counter = 0;
    loop {
        counter += 1;
        let mut outputs: VecDeque<_> = [Pulse::new(
            "button".into(),
            "broadcaster".into(),
            PulseValue::Low,
        )]
        .into();
        while let Some(pulse) = outputs.pop_front() {
            if pulse.value == PulseValue::High && pulse.src == "rv" {
                return counter.to_string();
            }

            if let Some(module) = modules.iter_mut().find(|m| m.name == pulse.dst) {
                module.process(&pulse, &mut outputs);
            }
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, ModuleDescriptor> {
    map(
        separated_pair(
            alt((
                map(alpha1, |name| (None, name)),
                map(pair(alt((char('%'), char('&'))), alpha1), |(t, name)| {
                    (Some(t), name)
                }),
            )),
            tag(" -> "),
            separated_list1(tag(", "), alpha1),
        ),
        |((class, name), outputs)| ModuleDescriptor {
            class,
            name,
            outputs,
            inputs: Vec::new(),
        },
    )(input)
}

struct ModuleDescriptor<'a> {
    class: Option<char>,
    name: &'a str,
    outputs: Vec<&'a str>,
    inputs: Vec<&'a str>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PulseValue {
    High,
    Low,
}

#[derive(Debug)]
struct Pulse {
    src: String,
    dst: String,
    value: PulseValue,
}

impl Pulse {
    fn new(src: String, dst: String, value: PulseValue) -> Self {
        Self { src, dst, value }
    }
}

enum ModuleState<'a> {
    Broadcaster,
    FlipFlop { enabled: bool },
    Conjunction { memory: Vec<(&'a str, PulseValue)> },
}

struct Module<'a> {
    name: &'a str,
    state: ModuleState<'a>,
    outputs: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn from_desscriptor(descriptor: &'a ModuleDescriptor) -> Self {
        let state = match descriptor.class {
            Some('%') => ModuleState::FlipFlop { enabled: false },
            Some('&') => ModuleState::Conjunction {
                memory: descriptor
                    .inputs
                    .iter()
                    .map(|&input| (input, PulseValue::Low))
                    .collect(),
            },
            None => ModuleState::Broadcaster,
            _ => unreachable!(),
        };
        Self {
            name: descriptor.name,
            state,
            outputs: descriptor.outputs.clone(),
        }
    }

    fn process(&mut self, pulse: &Pulse, output: &mut VecDeque<Pulse>) {
        match &mut self.state {
            ModuleState::Broadcaster => {
                self.broadcast(output, pulse.value);
            }
            ModuleState::FlipFlop { enabled } if pulse.value == PulseValue::Low => {
                *enabled = !*enabled;
                let out_value = if *enabled {
                    PulseValue::High
                } else {
                    PulseValue::Low
                };
                self.broadcast(output, out_value);
            }
            ModuleState::Conjunction { memory } => {
                memory.iter_mut().find(|m| m.0 == pulse.src).unwrap().1 = pulse.value;
                if memory.iter().all(|x| x.1 == PulseValue::High) {
                    self.broadcast(output, PulseValue::Low);
                } else {
                    self.broadcast(output, PulseValue::High);
                }
            }
            _ => (),
        }
    }

    fn broadcast(&self, output: &'a mut VecDeque<Pulse>, pulse_value: PulseValue) {
        for dst in &self.outputs {
            output.push_back(Pulse::new(
                self.name.to_string(),
                dst.to_string(),
                pulse_value,
            ));
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1_ex1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        let output = super::part1(input);

        assert_eq!(output, "32000000")
    }

    #[test]
    fn test_part1_ex2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        let output = super::part1(input);

        assert_eq!(output, "11687500")
    }
}
