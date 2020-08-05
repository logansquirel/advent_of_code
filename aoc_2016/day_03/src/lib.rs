use std::num::ParseIntError;
use std::str::FromStr;

pub fn puzzle_one(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter_map(|l| Triangle::from_str(l).ok())
        .filter(|t| t.is_valid())
        .count() as u32
}

pub fn puzzle_two(input: &str) -> u32 {
    let vec: Vec<u32> = input
        .trim()
        .lines()
        .flat_map(|l| l.split_whitespace())
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();
    let mut triangles: Vec<Triangle> = Vec::new();
    let len = vec.len() / 9;
    for start in 0..3 {
        let mut iter = vec.iter().skip(start).step_by(3);
        for _ in 0..len {
            let sides = [
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
            ];
            let triangle = Triangle { sides };
            //dbg!(&triangle);
            triangles.push(triangle);
        }
    }
    //dbg!(&triangles);
    triangles.iter().filter(|t| t.is_valid()).count() as u32
}

#[derive(Debug)]
struct Triangle {
    sides: [u32; 3],
}

impl Triangle {
    fn is_valid(&self) -> bool {
        let sides = self.sides;
        sides[0] < sides[1] + sides[2]
            && sides[2] < sides[0] + sides[1]
            && sides[1] < sides[2] + sides[0]
    }
}

impl FromStr for Triangle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<u32> = s
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        let sides = [vec[0], vec[1], vec[2]];
        Ok(Triangle { sides })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        assert!(!Triangle { sides: [5, 10, 25] }.is_valid())
    }
}
