use aoc_2019_day_02::puzzle_one;
use aoc_2019_day_02::puzzle_two;

#[test]
fn puzzle_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(3101878, puzzle_one(input));
}

#[test]
fn puzzle_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(8444, puzzle_two(input));
}
