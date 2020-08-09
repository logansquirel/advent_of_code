use std::collections::HashSet;

pub fn puzzle_one(input: &str) -> i32 {
    let square = input
        .trim()
        .parse::<i32>()
        .expect("impossible to parse input");
    let mut spiral = 1;
    let mut odd = 1;
    while odd * odd < square {
        spiral += 1;
        odd += 2;
    }
    if spiral == 1 {
        return 0;
    }
    let diff = odd * odd - square;
    let dist_to_corner = diff % (odd - 1);
    let dist_to_side_center = (dist_to_corner - spiral).abs() - 1;
    (spiral - 1) + dist_to_side_center
}

pub fn puzzle_two(input: &str) -> u32 {
    let limit = input
        .trim()
        .parse::<u32>()
        .expect("impossible to parse input");
    let mut x = 0;
    let mut y = 0;
    let mut square = Square { x, y, value: 1 };
    let mut set = HashSet::new();
    let mut direction = Direction::East;
    let mut spiral = 1;
    set.insert(square.clone());
    while square.value < limit {
        // start of a new spiral
        if x == (spiral - 1) && y == -(spiral - 1) {
            spiral += 1;
        }
        // change direction at corner
        else if x.abs() == (spiral - 1) && y.abs() == (spiral - 1)
            || x == (spiral - 1) && y == -(spiral - 2)
        {
            direction = direction.turn();
        }
        let pos = direction.step(x, y);
        x = pos.0;
        y = pos.1;
        let value = square_value(&set, x, y);
        square = Square { x, y, value };
        set.insert(square.clone());
    }
    square.value
}

fn square_value(set: &HashSet<Square>, x: i32, y: i32) -> u32 {
    set.iter()
        .filter(|&s| (s.x - x).abs() <= 1 && (s.y - y).abs() <= 1)
        .map(|s| s.value)
        .sum()
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn step(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::North => (x, y + 1),
            Direction::South => (x, y - 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
        }
    }

    fn turn(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Square {
    x: i32,
    y: i32,
    value: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_one_example_one() {
        assert_eq!(0, puzzle_one("1"));
    }

    #[test]
    fn puzzle_one_example_two() {
        assert_eq!(3, puzzle_one("12"));
    }

    #[test]
    fn puzzle_one_example_three() {
        assert_eq!(2, puzzle_one("23"));
    }

    #[test]
    fn puzzle_one_example_four() {
        assert_eq!(31, puzzle_one("1024"));
    }

    #[test]
    fn puzzle_two_test_one() {
        assert_eq!(1, puzzle_two("1"))
    }

    #[test]
    fn puzzle_two_test_two() {
        assert_eq!(4, puzzle_two("3"))
    }

    #[test]
    fn puzzle_two_test_three() {
        assert_eq!(23, puzzle_two("18"))
    }

    #[test]
    fn puzzle_two_test_four() {
        assert_eq!(142, puzzle_two("140"))
    }

    #[test]
    fn puzzle_two_test_five() {
        assert_eq!(806, puzzle_two("750"))
    }
}
