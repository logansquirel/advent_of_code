use aoc_yyyy_day_dd::puzzle_one;
use aoc_yyyy_day_dd::puzzle_two;
use aoc_yyyy_day_dd::Result;

#[test]
fn solution_puzzle_one() -> Result<()> {
    let input = include_str!("../input/input_one.dat");
    assert_eq!(String::from("foo"), puzzle_one(input)?);
    Ok(())
}

#[test]
fn solution_puzzle_two() -> Result<()> {
    let input = include_str!("../input/input_two.dat");
    assert_eq!(String::from("bar"), puzzle_two(input)?);
    Ok(())
}
