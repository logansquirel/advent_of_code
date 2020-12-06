use std::collections::HashSet;

pub fn part_one(input: &str) -> u32 {
    let mut position = Position::default();
    let mut seen = HashSet::new();
    seen.insert(position);
    for c in input.trim().chars() {
        position.step(c);
        seen.insert(position);
    }
    seen.len() as u32
}

pub fn part_two(input: &str) -> u32 {
    let mut position = Position::default();
    let mut seen = HashSet::new();
    seen.insert(position);
    // Santa
    for c in input.trim().chars().step_by(2) {
        position.step(c);
        seen.insert(position);
    }
    position = Position::default();
    // Robo-Santa
    for c in input.trim().chars().skip(1).step_by(2) {
        position.step(c);
        seen.insert(position);
    }
    seen.len() as u32
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(&mut self, c: char) {
        match c {
            '^' => self.north(),
            'v' => self.south(),
            '>' => self.east(),
            '<' => self.west(),
            _ => (),
        }
    }

    fn north(&mut self) {
        self.y += 1;
    }

    fn south(&mut self) {
        self.y -= 1;
    }

    fn east(&mut self) {
        self.x += 1;
    }

    fn west(&mut self) {
        self.x -= 1;
    }
}

#[test]
fn part_one_example_one() {
    assert_eq!(2, part_one(">"));
}

#[test]
fn part_one_example_two() {
    assert_eq!(4, part_one("^>v<"));
}

#[test]
fn part_one_example_three() {
    assert_eq!(2, part_one("^v^v^v^v^v"));
}

#[test]
fn part_two_example_one() {
    assert_eq!(3, part_two("^v"));
}

#[test]
fn part_two_example_two() {
    assert_eq!(3, part_two("^>v<"));
}

#[test]
fn part_two_example_three() {
    assert_eq!(11, part_two("^v^v^v^v^v"));
}
