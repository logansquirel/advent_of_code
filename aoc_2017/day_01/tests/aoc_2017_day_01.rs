use aoc_2017_day_01::part_one;
use aoc_2017_day_01::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(1119, part_one(input));
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(1420, part_two(input));
}
