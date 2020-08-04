use std::collections::HashSet;

pub fn puzzle_one(input: &str) -> u32 {
    let mut position = Position::default();
    let mut seen = HashSet::new();
    seen.insert(position);
    for c in input.trim().chars() {
        position.step(c);
        seen.insert(position);
    }
    seen.len() as u32
}

pub fn puzzle_two(input: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_one_example_one() {
        assert_eq!(2, puzzle_one(">"));
    }

    #[test]
    fn puzzle_one_example_two() {
        assert_eq!(4, puzzle_one("^>v<"));
    }

    #[test]
    fn puzzle_one_example_three() {
        assert_eq!(2, puzzle_one("^v^v^v^v^v"));
    }

    #[test]
    fn puzzle_two_example_one() {
        assert_eq!(3, puzzle_two("^v"));
    }

    #[test]
    fn puzzle_two_example_two() {
        assert_eq!(3, puzzle_two("^>v<"));
    }

    #[test]
    fn puzzle_two_example_three() {
        assert_eq!(11, puzzle_two("^v^v^v^v^v"));
    }
}
