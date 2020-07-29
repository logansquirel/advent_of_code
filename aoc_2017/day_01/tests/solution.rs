use aoc_2017_day_01::puzzle_one;
use aoc_2017_day_01::puzzle_two;

#[test]
fn solution_puzzle_one() {
    let input = include_str!("../input/input.dat");
    assert_eq!(1119, puzzle_one(input));
}

#[test]
fn solution_puzzle_two() {
    let input = include_str!("../input/input.dat");
    assert_eq!(1420, puzzle_two(input));
}
