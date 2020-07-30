use aoc_2016_day_02::puzzle_one;
use aoc_2016_day_02::puzzle_two;

#[test]
fn solution_puzzle_one() {
    let input = include_str!("../input/input.dat");
    assert_eq!(String::from("36629"), puzzle_one(input));
}

#[test]
fn solution_puzzle_two() {
    let input = include_str!("../input/input.dat");
    assert_eq!(String::from("99C3D"), puzzle_two(input));
}
