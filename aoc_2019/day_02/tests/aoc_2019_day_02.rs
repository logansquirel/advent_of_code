use aoc_2019_day_02::part_one;
use aoc_2019_day_02::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(3101878, part_one(input));
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(8444, part_two(input));
}
