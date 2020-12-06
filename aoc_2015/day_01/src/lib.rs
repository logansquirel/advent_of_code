pub fn part_one(input: &str) -> i32 {
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

pub fn part_two(input: &str) -> usize {
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
fn part_one_example_one() {
    assert_eq!(0, part_one("(())"));
    assert_eq!(0, part_one("()()"));
}

#[test]
fn part_one_example_two() {
    assert_eq!(3, part_one("((("));
    assert_eq!(3, part_one("(()(()("));
}

#[test]
fn part_one_example_three() {
    assert_eq!(3, part_one("))((((("));
}

#[test]
fn part_one_example_four() {
    assert_eq!(-1, part_one("())"));
    assert_eq!(-1, part_one("))("));
}

#[test]
fn part_one_example_five() {
    assert_eq!(-3, part_one(")))"));
    assert_eq!(-3, part_one(")())())"));
}

#[test]
fn part_two_example_one() {
    assert_eq!(1, part_two(")"));
}

#[test]
fn part_two_example_two() {
    assert_eq!(5, part_two("()())"));
}
