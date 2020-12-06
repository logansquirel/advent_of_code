pub fn part_one(input: &str) -> u32 {
    let vec: Vec<u32> = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    let mut sum = 0;
    let len = vec.len();
    for idx in 0..(len - 1) {
        if vec[idx] == vec[idx + 1] {
            sum += vec[idx];
        }
    }
    if vec[len - 1] == vec[0] {
        sum += vec[0];
    }
    sum
}

pub fn part_two(input: &str) -> u32 {
    let vec: Vec<u32> = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    let mut sum = 0;
    let len = vec.len();
    for i in 0..len {
        let idx = (i + len / 2) % len;
        if vec[i] == vec[idx] {
            sum += vec[i]
        }
    }
    sum
}

#[test]
fn part_one_example_one() {
    assert_eq!(3, part_one("1122"));
}

#[test]
fn part_one_example_two() {
    assert_eq!(4, part_one("1111"));
}

#[test]
fn part_one_example_three() {
    assert_eq!(0, part_one("1234"));
}

#[test]
fn part_one_example_four() {
    assert_eq!(9, part_one("91212129"));
}

#[test]
fn part_two_example_one() {
    assert_eq!(6, part_two("1212"));
}

#[test]
fn part_two_example_two() {
    assert_eq!(0, part_two("1221"));
}

#[test]
fn part_two_example_three() {
    assert_eq!(4, part_two("123425"));
}

#[test]
fn part_two_example_four() {
    assert_eq!(12, part_two("123123"));
}

#[test]
fn part_two_example_five() {
    assert_eq!(4, part_two("12131415"));
}
