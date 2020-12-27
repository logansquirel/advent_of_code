use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

fn initialize_lights<L: Light>() -> HashMap<Position, L> {
    HashMap::with_capacity(1_000_000)
}

fn set_up_lights<L: Light>(instructions: &str, lights: &mut HashMap<Position, L>) {
    let regex = Regex::new(r"(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    for instruction in instructions.lines() {
        follow_instruction(instruction, &regex, lights);
    }
}

fn follow_instruction<L: Light>(
    instruction: &str,
    regex: &Regex,
    lights: &mut HashMap<Position, L>,
) {
    let captures = regex
        .captures(instruction)
        .unwrap_or_else(|| panic!("invalid instruction format: '{}'", instruction));
    let operation = Operation::from(&captures[1]);
    let x1: u32 = captures[2].parse().unwrap();
    let y1: u32 = captures[3].parse().unwrap();
    let x2: u32 = captures[4].parse().unwrap();
    let y2: u32 = captures[5].parse().unwrap();
    match operation {
        Operation::TurnOff => {
            operate_lights(x1..=x2, y1..=y2, lights, Light::turn_off);
        }
        Operation::TurnOn => {
            operate_lights(x1..=x2, y1..=y2, lights, Light::turn_on);
        }
        Operation::Toggle => {
            operate_lights(x1..=x2, y1..=y2, lights, Light::toggle);
        }
    }
}

fn operate_lights<L: Light>(
    x_range: RangeInclusive<u32>,
    y_range: RangeInclusive<u32>,
    lights: &mut HashMap<Position, L>,
    operation: fn(&mut L),
) {
    for x in x_range {
        for y in y_range.clone() {
            let pos = Position::new(x, y);
            let light = lights.entry(pos).or_insert_with(L::new);
            operation(light);
        }
    }
}
trait Light {
    fn new() -> Self;
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn toggle(&mut self);
    fn is_on(&self) -> bool;
}

#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Self {
        assert!(x < 1000);
        assert!(y < 1000);
        Position { x, y }
    }
}

enum Operation {
    TurnOff,
    TurnOn,
    Toggle,
}

impl From<&str> for Operation {
    fn from(op: &str) -> Self {
        match op {
            "turn off" => Self::TurnOff,
            "turn on" => Self::TurnOn,
            "toggle" => Self::Toggle,
            _ => unreachable!(),
        }
    }
}
