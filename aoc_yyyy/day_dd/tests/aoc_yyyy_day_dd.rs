use aoc_yyyy_day_dd::part_one;
use aoc_yyyy_day_dd::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(String::from("foo"), part_one(input));
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(String::from("bar"), part_two(input));
}
