use aoc_2018_day_02::puzzle_one;
use aoc_2018_day_02::puzzle_two;

#[test]
fn puzzle_one_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(5704, puzzle_one(input));
}

#[test]
fn puzzle_two_answer() {
    let input = include_str!("../input/input.dat");
    assert_eq!(String::from("umdryabviapkozistwcnihjqx"), puzzle_two(input));
}
