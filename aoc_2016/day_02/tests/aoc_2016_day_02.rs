use aoc_2016_day_02::part_one;
use aoc_2016_day_02::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(String::from("36629"), part_one(input));
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(String::from("99C3D"), part_two(input));
}
