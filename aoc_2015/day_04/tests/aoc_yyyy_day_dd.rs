use aoc_2015_day_04::part_one;
use aoc_2015_day_04::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(117946, part_one(input));
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(3938038, part_two(input))
}
