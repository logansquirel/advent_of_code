use std::io::Read;

use aoc_2015_day_01::puzzle_one;
use aoc_2015_day_01::puzzle_two;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    println!("Advent of Code 2015-01");
    println!("------ Puzzle 1 ------");
    println!("{}", puzzle_one(&input));
    println!();
    println!("------ Puzzle 2 ------");
    println!("{}", puzzle_two(&input));
    println!();
    Ok(())
}
