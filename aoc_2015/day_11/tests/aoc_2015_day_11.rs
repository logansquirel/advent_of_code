use aoc_2015_day_11::part_one;
use aoc_2015_day_11::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(part_one(input), "hepxxyzz");
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(part_two(input), "heqaabcc")
}
