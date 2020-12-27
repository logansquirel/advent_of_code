use aoc_2015_day_08::part_one;
use aoc_2015_day_08::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(1350, part_one(input));
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(2085, part_two(input));
}
