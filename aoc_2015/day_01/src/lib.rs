pub fn puzzle_one(input: &str) -> i32 {
    let mut floor = 0;
    for char in input.trim().chars() {
        match char {
            ')' => floor -= 1,
            '(' => floor += 1,
            _ => continue,
        }
    }
    floor
}

pub fn puzzle_two(input: &str) -> usize {
    let mut floor = 0;
    for (pos, char) in input.trim().char_indices() {
        match char {
            ')' => floor -= 1,
            '(' => floor += 1,
            _ => continue,
        }
        if floor == -1 {
            return pos + 1;
        }
    }
    panic!("basement not found")
}

#[test]
fn puzzle_one_example_one() {
    assert_eq!(0, puzzle_one("(())"));
    assert_eq!(0, puzzle_one("()()"));
}

#[test]
fn puzzle_one_example_two() {
    assert_eq!(3, puzzle_one("((("));
    assert_eq!(3, puzzle_one("(()(()("));
}

#[test]
fn puzzle_one_example_three() {
    assert_eq!(3, puzzle_one("))((((("));
}

#[test]
fn puzzle_one_example_four() {
    assert_eq!(-1, puzzle_one("())"));
    assert_eq!(-1, puzzle_one("))("));
}

#[test]
fn puzzle_one_example_five() {
    assert_eq!(-3, puzzle_one(")))"));
    assert_eq!(-3, puzzle_one(")())())"));
}

#[test]
fn puzzle_two_example_one() {
    assert_eq!(1, puzzle_two(")"));
}

#[test]
fn puzzle_two_example_two() {
    assert_eq!(5, puzzle_two("()())"));
}
