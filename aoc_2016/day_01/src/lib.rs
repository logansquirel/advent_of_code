use std::collections::HashSet;

pub fn part_one(input: &str) -> u32 {
    let mut pos = Position {
        coordinates: Coordinates { x: 0, y: 0 },
        dir: Direction::North,
    };
    for instruction in input.trim().split(", ") {
        match instruction.chars().next() {
            Some('L') => pos.left(),
            Some('R') => pos.right(),
            _ => panic!("invalid instruction"),
        }
        let count: i32 = instruction[1..].parse().expect("impossible to parse count");
        pos.shift(count);
    }
    pos.coordinates.distance()
}

pub fn part_two(input: &str) -> u32 {
    let mut pos = Position {
        coordinates: Coordinates { x: 0, y: 0 },
        dir: Direction::North,
    };
    let mut seen = HashSet::new();
    for instruction in input.trim().split(", ") {
        match instruction.chars().next() {
            Some('L') => pos.left(),
            Some('R') => pos.right(),
            _ => panic!("invalid instruction"),
        }
        let count: i32 = instruction[1..].parse().expect("impossible to parse count");
        for _ in 0..count {
            pos.step();
            if seen.contains(&pos.coordinates) {
                return pos.coordinates.distance();
            } else {
                seen.insert(pos.coordinates);
            }
        }
    }
    panic!("Easter Bunny HQ not found")
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    coordinates: Coordinates,
    dir: Direction,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    East,
    North,
    South,
    West,
}

impl Position {
    fn left(&mut self) {
        self.dir = match self.dir {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn right(&mut self) {
        self.dir = match self.dir {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn shift(&mut self, count: i32) {
        match self.dir {
            Direction::East => self.coordinates.shift(count, 0),
            Direction::North => self.coordinates.shift(0, count),
            Direction::South => self.coordinates.shift(0, -count),
            Direction::West => self.coordinates.shift(-count, 0),
        }
    }

    fn step(&mut self) {
        match self.dir {
            Direction::East => self.coordinates.shift(1, 0),
            Direction::North => self.coordinates.shift(0, 1),
            Direction::South => self.coordinates.shift(0, -1),
            Direction::West => self.coordinates.shift(-1, 0),
        }
    }
}

impl Coordinates {
    fn distance(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }

    fn shift(&mut self, xshift: i32, yshift: i32) {
        self.x += xshift;
        self.y += yshift;
    }
}

#[test]
fn part_one_example_one() {
    assert_eq!(5, part_one("R2, L3"));
}

#[test]
fn part_one_example_two() {
    assert_eq!(2, part_one("R2, R2, R2"));
}

#[test]
fn part_one_example_three() {
    assert_eq!(12, part_one("R5, L5, R5, R3"));
}

#[test]
fn part_two_example() {
    assert_eq!(4, part_two("R8, R4, R4, R8"));
}
