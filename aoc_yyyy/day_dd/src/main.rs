use std::io::Read;

use aoc_yyyy_day_dd::puzzle_one;
use aoc_yyyy_day_dd::puzzle_two;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read input");
    println!("Advent of Code yyyy-dd");
    println!("------ Puzzle 1 ------");
    println!("{}", puzzle_one(&input));
    println!();
    println!("------ Puzzle 2 ------");
    println!("{}", puzzle_two(&input));
    println!();
}
