use aoc_2015_day_03::puzzle_one;
use aoc_2015_day_03::puzzle_two;

#[test]
fn puzzle_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(2572, puzzle_one(input));
}

#[test]
fn puzzle_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(2631, puzzle_two(input));
}
