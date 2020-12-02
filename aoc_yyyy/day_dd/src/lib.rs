pub fn puzzle_one(input: &str) -> String {
    String::from(input[..3].trim())
}

pub fn puzzle_two(input: &str) -> String {
    String::from(input[3..].trim())
}

#[test]
fn puzzle_one_example() {
    assert_eq!(String::from("foo"), puzzle_one("foobar"));
}

#[test]
fn puzzle_two_example() {
    assert_eq!(String::from("bar"), puzzle_two("foobar"));
}
