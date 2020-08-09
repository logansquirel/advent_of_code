use aoc_2017_day_03::puzzle_one;
use aoc_2017_day_03::puzzle_two;

#[test]
fn solution_puzzle_one() {
    let input = include_str!("../input/input.dat");
    assert_eq!(480, puzzle_one(input));
}

#[test]
fn solution_puzzle_two() {
    let input = include_str!("../input/input.dat");
    assert_eq!(349975, puzzle_two(input));
}
