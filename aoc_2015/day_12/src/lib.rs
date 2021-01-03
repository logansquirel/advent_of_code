pub fn part_one(input: &str) -> i32 {
    sum(input.trim())
}

pub fn part_two(input: &str) -> i32 {
    let input = ignore(input.trim());
    sum(&input)
}

fn sum(input: &str) -> i32 {
    let mut iter = input.trim().chars();
    let mut sum = 0;
    while let Some(char) = iter.next() {
        match char {
            '-' | '0'..='9' => {
                let mut number = String::from(char);
                while let Some(digit @ '0'..='9') = iter.next() {
                    number.push(digit);
                }
                sum += number.parse::<i32>().unwrap();
            }
            _ => continue,
        }
    }
    sum
}

fn ignore(input: &str) -> String {
    let pattern = ":\"red\"";
    let mut src = input.to_string();
    while let Some(idx) = src.find(pattern) {
        let mut opened = 1;
        let mut before = 0;
        let mut iter = src[..idx].chars().rev();
        while opened != 0 {
            match iter.next() {
                Some('}') => opened += 1,
                Some('{') => opened -= 1,
                Some(_) => (),
                None => panic!("invalid JSON"),
            }
            before += 1;
        }
        let start = idx - before;
        let mut closed = 1;
        let mut after = 0;
        let mut iter = src[idx + pattern.len()..].chars();
        while closed != 0 {
            match iter.next() {
                Some('}') => closed -= 1,
                Some('{') => closed += 1,
                Some(_) => (),
                None => panic!("invalid JSON"),
            }
            after += 1;
        }
        let end = idx + pattern.len() + after;
        src.replace_range(start..end, "_");
    }
    src
}

#[test]
fn part_one_example_one() {
    assert_eq!(part_one(r#"[1,2,3]"#), 6);
    assert_eq!(part_one(r#"{"a":2,"b":4}"#), 6);
}

#[test]
fn part_one_example_two() {
    assert_eq!(part_one(r#"[[[3]]]"#), 3);
    assert_eq!(part_one(r#"{"a":{"b":4},"c":-1}"#), 3);
}

#[test]
fn part_one_example_three() {
    assert_eq!(part_one(r#"{"a":[-1,1]}"#), 0);
    assert_eq!(part_one(r#"[-1,{"a":1}]"#), 0);
}

#[test]
fn part_one_example_four() {
    assert_eq!(part_one(r#"[]"#), 0);
    assert_eq!(part_one(r#"{}"#), 0);
}

#[test]
fn part_two_example_one() {
    assert_eq!(part_two(r#"[1,2,3]"#), 6);
}

#[test]
fn part_two_example_two() {
    assert_eq!(part_two(r#"[1,{"c":"red","b":2},3]"#), 4);
}

#[test]
fn part_two_example_three() {
    assert_eq!(part_two(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
}

#[test]
fn part_two_example_four() {
    assert_eq!(part_two(r#"[1,"red",5]"#), 6);
}
