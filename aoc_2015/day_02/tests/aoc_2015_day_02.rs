use aoc_2015_day_02::puzzle_one;
use aoc_2015_day_02::puzzle_two;

#[test]
fn puzzle_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(1606483, puzzle_one(input));
}

#[test]
fn puzzle_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(3842356, puzzle_two(input));
}
