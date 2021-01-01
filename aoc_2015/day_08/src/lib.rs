pub fn part_one(input: &str) -> usize {
    let mut code = 0;
    let mut memory = 0;
    for line in input.trim().lines() {
        code += code_count(line);
        memory += memory_count(line);
    }
    code - memory
}

pub fn part_two(input: &str) -> usize {
    let mut encode = 0;
    let mut code = 0;
    for line in input.trim().lines() {
        encode += encode_count(line);
        code += code_count(line);
    }
    encode - code
}

fn code_count(str: &str) -> usize {
    str.len()
}

fn encode_count(str: &str) -> usize {
    let str = str.replace("\\", "\\\\");
    let str = str.replace('"', "\\\"");
    str.len() + 2
}

fn memory_count(str: &str) -> usize {
    let str = str.strip_prefix('"').unwrap();
    let str = str.strip_suffix('"').unwrap();
    let mut count = 0;
    let mut iter = str.chars();
    loop {
        match iter.next() {
            Some('\\') => match iter.next() {
                Some('"') | Some('\\') => count += 1,
                Some('x') => {
                    iter.next();
                    iter.next();
                    count += 1
                }
                _ => unreachable!("invalid escape sequence"),
            },
            Some(_) => count += 1,
            None => break,
        }
    }
    count
}

#[test]
fn part_one_example_one() {
    let str = r#""""#;
    assert_eq!(code_count(str), 2);
    assert_eq!(memory_count(str), 0);
}

#[test]
fn part_one_example_two() {
    let str = r#""abc""#;
    assert_eq!(code_count(str), 5);
    assert_eq!(memory_count(str), 3);
}

#[test]
fn part_one_example_three() {
    let str = r#""aaa\"aaa""#;
    assert_eq!(code_count(str), 10);
    assert_eq!(memory_count(str), 7);
}

#[test]
fn part_one_example_four() {
    let str = r#""\x27""#;
    assert_eq!(code_count(str), 6);
    assert_eq!(memory_count(str), 1);
}

#[test]
fn part_one_examples() {
    let input = r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#;
    assert_eq!(part_one(input), 12)
}

#[test]
fn part_two_example_one() {
    let str = r#""""#;
    assert_eq!(encode_count(str), 6);
    assert_eq!(code_count(str), 2);
}

#[test]
fn part_two_example_two() {
    let str = r#""abc""#;
    assert_eq!(encode_count(str), 9);
    assert_eq!(code_count(str), 5);
}

#[test]
fn part_two_example_three() {
    let str = r#""aaa\"aaa""#;
    assert_eq!(encode_count(str), 16);
    assert_eq!(code_count(str), 10);
}

#[test]
fn part_two_example_four() {
    let str = r#""\x27""#;
    assert_eq!(encode_count(str), 11);
    assert_eq!(code_count(str), 6);
}

#[test]
fn part_two_examples() {
    let input = r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#;
    assert_eq!(part_two(input), 19)
}
