use aoc_2018_day_01::part_one;
use aoc_2018_day_01::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(556, part_one(input));
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(448, part_two(input));
}
