use aoc_2018_day_02::puzzle_one;
use aoc_2018_day_02::puzzle_two;

#[test]
fn solution_puzzle_one() {
    let input = include_str!("../input/input.dat");
    assert_eq!(5704, puzzle_one(input));
}

#[test]
fn solution_puzzle_two() {
    let input = include_str!("../input/input.dat");
    assert_eq!(String::from("umdryabviapkozistwcnihjqx"), puzzle_two(input));
}
