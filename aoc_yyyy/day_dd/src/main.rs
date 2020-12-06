use std::io::Read;

use aoc_yyyy_day_dd::part_one;
use aoc_yyyy_day_dd::part_two;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read input");
    println!("Advent of Code yyyy-dd");
    println!("------ Part 1 ------");
    println!("{}", part_one(&input));
    println!();
    println!("------ Part 2 ------");
    println!("{}", part_two(&input));
    println!();
}
