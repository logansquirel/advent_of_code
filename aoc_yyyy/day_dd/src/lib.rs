pub fn part_one(input: &str) -> String {
    String::from(input[..3].trim())
}

pub fn part_two(input: &str) -> String {
    String::from(input[3..].trim())
}

#[test]
fn part_one_example() {
    assert_eq!(String::from("foo"), part_one("foobar"));
}

#[test]
fn part_two_example() {
    assert_eq!(String::from("bar"), part_two("foobar"));
}
