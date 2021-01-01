pub fn part_one(input: &str) -> usize {
    let mut buffer = input.trim().to_string();
    for _ in 0..40 {
        buffer = look_and_say(&buffer);
    }
    buffer.len()
}

pub fn part_two(input: &str) -> usize {
    let mut buffer = input.trim().to_string();
    for _ in 0..50 {
        buffer = look_and_say(&buffer);
    }
    buffer.len()
}

fn look_and_say(input: &str) -> String {
    let mut iter = input.chars().peekable();
    let mut buf = String::new();
    while let Some(digit) = iter.next() {
        assert!(
            digit.is_ascii_digit(),
            "invalid digit '{}'",
            digit.escape_default()
        );
        let mut count = 1;
        while iter.peek() == Some(&digit) {
            iter.next();
            count += 1;
        }
        buf.push_str(&count.to_string());
        buf.push(digit);
    }
    buf
}

#[test]
fn example() {
    assert_eq!(look_and_say("1"), "11");
    assert_eq!(look_and_say("11"), "21");
    assert_eq!(look_and_say("21"), "1211");
    assert_eq!(look_and_say("1211"), "111221");
    assert_eq!(look_and_say("111221"), "312211");
}
