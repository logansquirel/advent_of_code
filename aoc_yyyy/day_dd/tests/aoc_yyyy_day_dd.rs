use aoc_yyyy_day_dd::puzzle_one;
use aoc_yyyy_day_dd::puzzle_two;

#[test]
fn puzzle_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(String::from("foo"), puzzle_one(input));
}

#[test]
fn puzzle_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(String::from("bar"), puzzle_two(input));
}
