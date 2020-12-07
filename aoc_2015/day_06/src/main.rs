use std::io::Read;

use aoc_2015_day_06::part_one;
use aoc_2015_day_06::part_two;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read input");
    println!("Advent of Code 2015-06");
    println!("------ Part 1 ------");
    println!("{}", part_one(&input));
    println!();
    println!("------ Part 2 ------");
    println!("{}", part_two(&input));
    println!();
}
