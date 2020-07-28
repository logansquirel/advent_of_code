use aoc_2017_day_01::puzzle_one;
use aoc_2017_day_01::puzzle_two;
use aoc_2017_day_01::Result;

#[test]
fn solution_puzzle_one() -> Result<()> {
    let input = include_str!("../input/input.dat");
    assert_eq!(1119, puzzle_one(input));
    Ok(())
}

#[test]
fn solution_puzzle_two() -> Result<()> {
    let input = include_str!("../input/input.dat");
    assert_eq!(1420, puzzle_two(input));
    Ok(())
}
