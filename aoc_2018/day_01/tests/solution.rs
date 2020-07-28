use aoc_2018_day_01::puzzle_one;
use aoc_2018_day_01::puzzle_two;

#[test]
fn solution_puzzle_one() {
    let input = include_str!("../input/input.dat");
    assert_eq!(556, puzzle_one(input));
}

#[test]
fn solution_puzzle_two() {
    let input = include_str!("../input/input.dat");
    assert_eq!(448, puzzle_two(input));
}
