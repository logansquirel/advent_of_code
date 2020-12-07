use aoc_2015_day_06::part_one;
use aoc_2015_day_06::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(543903, part_one(input));
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(14687245, part_two(input));
}
