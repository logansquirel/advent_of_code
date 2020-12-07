use crate::Light;
use crate::{initialize_lights, set_up_lights};

pub fn part_one(input: &str) -> u32 {
    let mut lights = initialize_lights::<Light1>();
    set_up_lights(input.trim(), &mut lights);
    lights.values().filter(|x| x.is_on()).count() as u32
}

struct Light1 {
    state: State,
}

enum State {
    On,
    Off,
}

impl super::Light for Light1 {
    fn new() -> Self {
        Light1 { state: State::Off }
    }

    fn turn_on(&mut self) {
        self.state = State::On
    }

    fn turn_off(&mut self) {
        self.state = State::Off
    }

    fn toggle(&mut self) {
        match self.state {
            State::Off => self.state = State::On,
            State::On => self.state = State::Off,
        }
    }

    fn is_on(&self) -> bool {
        match self.state {
            State::Off => false,
            State::On => true,
        }
    }
}

#[test]
fn part_one_example_one() {
    let input = "turn on 0,0 through 999,999";
    assert_eq!(1_000_000, part_one(input));
}

#[test]
fn part_one_example_two() {
    let input = "toggle 0,0 through 999,0";
    assert_eq!(1000, part_one(input));
}

#[test]
fn part_one_example_three() {
    let input = "turn off 499,499 through 500,500";
    assert_eq!(0, part_one(input));
}

#[test]
fn part_one_examples() {
    let input = "turn on 0,0 through 999,999\n\
    toggle 0,0 through 999,0\n\
    turn off 499,499 through 500,500\n\
    ";
    assert_eq!(1_000_000 - 1000 - 4, part_one(input));
}
