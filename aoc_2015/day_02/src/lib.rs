use std::num::ParseIntError;
use std::str::FromStr;

pub fn puzzle_one(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter_map(|l| Present::from_str(l).ok())
        .map(|p| p.paper())
        .sum()
}

pub fn puzzle_two(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter_map(|l| Present::from_str(l).ok())
        .map(|p| p.ribbon())
        .sum()
}

#[derive(Debug)]
struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(length: u32, width: u32, height: u32) -> Self {
        Present {
            length,
            width,
            height,
        }
    }

    fn paper(&self) -> u32 {
        let surfaces = vec![
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        let area: u32 = surfaces.iter().map(|x| 2 * x).sum();
        let smallest_side = *surfaces.iter().min().unwrap();
        area + smallest_side
    }

    fn ribbon(&self) -> u32 {
        let volume = self.length * self.width * self.height;
        let perimeters = vec![
            2 * (self.length + self.width),
            2 * (self.width + self.height),
            2 * (self.height + self.length),
        ];
        let smallest_perimeter = *perimeters.iter().min().unwrap();
        volume + smallest_perimeter
    }
}

impl FromStr for Present {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sides: Vec<&str> = s.trim().split('x').collect();
        let length = sides[0].parse::<u32>()?;
        let width = sides[1].parse::<u32>()?;
        let height = sides[2].parse::<u32>()?;
        Ok(Present::new(length, width, height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_one_example_one() {
        assert_eq!(58, puzzle_one("2x3x4"));
    }

    #[test]
    fn puzzle_one_example_two() {
        assert_eq!(43, puzzle_one("1x1x10"));
    }

    #[test]
    fn puzzle_two_example_one() {
        assert_eq!(34, puzzle_two("2x3x4"));
    }

    #[test]
    fn puzzle_two_example_two() {
        assert_eq!(14, puzzle_two("1x1x10"));
    }
}
