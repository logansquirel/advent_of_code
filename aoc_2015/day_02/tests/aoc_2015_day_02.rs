use aoc_2015_day_02::part_one;
use aoc_2015_day_02::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(1606483, part_one(input));
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(3842356, part_two(input));
}
