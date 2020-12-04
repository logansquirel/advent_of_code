use aoc_2017_day_03::puzzle_one;
use aoc_2017_day_03::puzzle_two;

#[test]
fn puzzle_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(480, puzzle_one(input));
}

#[test]
fn puzzle_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(349975, puzzle_two(input));
}
