use aoc_2015_day_13::part_one;
use aoc_2015_day_13::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(part_one(input), 733);
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(part_two(input), 725);
}
