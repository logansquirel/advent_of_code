pub fn part_one(input: &str) -> u32 {
    let mut count = 0;
    for string in input.trim().lines() {
        if is_nice_1(string) {
            count += 1;
        }
    }
    count
}

pub fn part_two(input: &str) -> u32 {
    let mut count = 0;
    for string in input.trim().lines() {
        if is_nice_2(string) {
            count += 1
        }
    }
    count
}

fn is_nice_1(string: &str) -> bool {
    property_1(string) && property_2(string) && property_3(string)
}

fn is_nice_2(string: &str) -> bool {
    property_4(string) && property_5(string)
}

fn property_1(string: &str) -> bool {
    let mut vowels = 0;
    for c in string.chars() {
        if "aeiou".contains(c) {
            vowels += 1;
        }
    }
    vowels >= 3
}

fn property_2(string: &str) -> bool {
    let mut iter = string.chars().peekable();
    while let Some(c) = iter.next() {
        match iter.peek() {
            Some(x) if *x == c => return true,
            _ => continue,
        }
    }
    false
}

fn property_3(string: &str) -> bool {
    !(string.contains("ab")
        || string.contains("cd")
        || string.contains("pq")
        || string.contains("xy"))
}

fn property_4(string: &str) -> bool {
    for i in 2..string.len() {
        if string[i..].contains(&string[i - 2..i]) {
            return true;
        }
    }
    false
}

fn property_5(string: &str) -> bool {
    let bytes = string.as_bytes();
    for i in 0..bytes.len() - 2 {
        if bytes[i] == bytes[i + 2] {
            return true;
        }
    }
    false
}

#[test]
fn property_1_examples() {
    assert!(property_1("aei"));
    assert!(property_1("xazegov"));
    assert!(property_1("aeiouaeiouaeiou"));
}

#[test]
fn property_2_examples() {
    assert!(property_2("xx"));
    assert!(property_2("abcdde"));
    assert!(property_2("aabbccdd"));
}

#[test]
fn property_4_examples() {
    assert!(property_4("xyxy"));
    assert!(property_4("aabcdefgaa"));
    assert!(!property_4("aaa"));
}

#[test]
fn property_5_examples() {
    assert!(property_5("xyx"));
    assert!(property_5("abcdefeghi"));
    assert!(property_5("aaa"));
}

#[test]
fn part_one_example_one() {
    assert!(is_nice_1("ugknbfddgicrmopn"));
}

#[test]
fn part_one_example_two() {
    assert!(is_nice_1("aaa"));
}

#[test]
fn part_one_example_three() {
    assert!(!is_nice_1("jchzalrnumimnmhp"))
}

#[test]
fn part_one_example_four() {
    assert!(!is_nice_1("haegwjzuvuyypxyu"))
}

#[test]
fn part_one_example_five() {
    assert!(!is_nice_1("dvszwmarrgswjxmb"))
}

#[test]
fn part_two_example_one() {
    assert!(is_nice_2("qjhvhtzxzqqjkmpb"));
}

#[test]
fn part_two_example_two() {
    assert!(is_nice_2("xxyxx"));
}

#[test]
fn part_two_example_three() {
    assert!(!is_nice_2("uurcxstgmygtbstg"))
}

#[test]
fn part_two_example_four() {
    assert!(!is_nice_2("ieodomkazucvgmuy"))
}
