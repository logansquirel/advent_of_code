use std::collections::HashSet;

pub fn puzzle_one(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .filter_map(|l| l.parse::<i32>().ok())
        .sum()
}

pub fn puzzle_two(input: &str) -> i32 {
    let mut seen = HashSet::new();
    let mut iter = input
        .trim()
        .lines()
        .filter_map(|l| l.parse::<i32>().ok())
        .cycle();
    let mut frequency = 0;
    loop {
        if seen.contains(&frequency) {
            break frequency;
        } else {
            seen.insert(frequency);
        }
        frequency += iter.next().unwrap();
    }
}

#[test]
fn puzzle_one_example_one() {
    assert_eq!(3, puzzle_one("+1\n-2\n+3\n+1"));
}

#[test]
fn puzzle_one_example_two() {
    assert_eq!(3, puzzle_one("+1\n+1\n+1"));
}

#[test]
fn puzzle_one_example_three() {
    assert_eq!(0, puzzle_one("+1\n+1\n-2"));
}

#[test]
fn puzzle_one_example_four() {
    assert_eq!(-6, puzzle_one("-1\n-2\n-3"));
}

#[test]
fn puzzle_two_example_one() {
    assert_eq!(0, puzzle_two("+1\n-1"));
}

#[test]
fn puzzle_two_example_two() {
    assert_eq!(10, puzzle_two("+3\n+3\n+4\n-2\n-4"));
}

#[test]
fn puzzle_two_example_three() {
    assert_eq!(5, puzzle_two("-6\n+3\n+8\n+5\n-6"));
}

#[test]
fn puzzle_two_example_four() {
    assert_eq!(14, puzzle_two("+7\n+7\n-2\n-7\n-4"));
}
