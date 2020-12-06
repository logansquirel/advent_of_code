use aoc_2018_day_02::part_one;
use aoc_2018_day_02::part_two;

#[test]
fn part_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(5704, part_one(input));
}

#[test]
fn part_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(String::from("umdryabviapkozistwcnihjqx"), part_two(input));
}
