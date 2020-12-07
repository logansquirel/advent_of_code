use crate::Light;
use crate::{initialize_lights, set_up_lights};

pub fn part_two(input: &str) -> u32 {
    let mut lights = initialize_lights::<Light2>();
    set_up_lights(input.trim(), &mut lights);
    lights
        .values()
        .filter(|x| x.is_on())
        .map(|x| x.brightness)
        .sum::<u32>()
}

struct Light2 {
    brightness: u32,
}

impl super::Light for Light2 {
    fn new() -> Self {
        Light2 { brightness: 0 }
    }

    fn turn_on(&mut self) {
        self.brightness += 1;
    }

    fn turn_off(&mut self) {
        self.brightness = self.brightness.saturating_sub(1);
    }

    fn toggle(&mut self) {
        self.brightness += 2;
    }

    fn is_on(&self) -> bool {
        self.brightness > 0
    }
}

#[test]
fn part_two_example_one() {
    let input = "turn on 0,0 through 0,0";
    assert_eq!(1, part_two(input));
}

#[test]
fn part_two_example_two() {
    let input = "toggle 0,0 through 999,999";
    assert_eq!(2_000_000, part_two(input));
}

#[test]
fn part_two_examples() {
    let input = "toggle 0,0 through 999,999\n\
    turn on 0,0 through 0,0";
    assert_eq!(2_000_001, part_two(input));
}
