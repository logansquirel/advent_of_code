use std::collections::HashSet;

pub fn part_one(input: &str) -> String {
    next_password(input.trim())
}

pub fn part_two(input: &str) -> String {
    let password = next_password(input.trim());
    next_password(&password)
}

fn next_password(password: &str) -> String {
    let mut password = password.to_string();
    loop {
        password = next_pwd(&password);
        if is_valid_pwd(&password) {
            break password;
        }
    }
}

fn next_pwd(password: &str) -> String {
    let mut iter = password.bytes().rev();
    let mut buf = Vec::new();
    match iter.next() {
        Some(byte @ b'a'..=b'g')
        | Some(byte @ b'i'..=b'j')
        | Some(byte @ b'l'..=b'm')
        | Some(byte @ b'o'..=b'y') => {
            buf.push(byte + 1);
        }
        // Optimization: do not generate password with illegal characters ('i', 'l' and 'o')
        Some(byte @ b'h') | Some(byte @ b'k') | Some(byte @ b'n') => {
            buf.push(byte + 2);
        }
        Some(b'z') => {
            buf.push(b'a');
            loop {
                match iter.next() {
                    Some(b'z') => buf.push(b'a'),
                    Some(byte @ b'a'..=b'g')
                    | Some(byte @ b'i'..=b'j')
                    | Some(byte @ b'l'..=b'm')
                    | Some(byte @ b'o'..=b'y') => {
                        buf.push(byte + 1);
                        break;
                    }
                    Some(byte @ b'h') | Some(byte @ b'k') | Some(byte @ b'n') => {
                        buf.push(byte + 2);
                        break;
                    }
                    None => {
                        buf.insert(0, b'a');
                        break;
                    }
                    _ => unreachable!("invalid character"),
                }
            }
        }
        None => unreachable!("empty password"),
        _ => unreachable!("invalid character"),
    }
    for byte in iter {
        assert!(byte >= b'a');
        assert!(byte <= b'z');
        buf.push(byte);
    }
    let bytes = buf.into_iter().rev().collect();
    String::from_utf8(bytes).expect("oinvalid UTF-8")
}

fn is_valid_pwd(password: &str) -> bool {
    requirement_1(password) & requirement_2(password) & requirement_3(password)
}

fn requirement_1(password: &str) -> bool {
    password
        .as_bytes()
        .windows(3)
        .any(|x| x[0] + 1 == x[1] && x[1] + 1 == x[2])
}

fn requirement_2(password: &str) -> bool {
    !(password.contains('i') | password.contains('o') | password.contains('l'))
}

fn requirement_3(password: &str) -> bool {
    let set: HashSet<_> = password
        .as_bytes()
        .windows(2)
        .filter(|x| x[0] == x[1])
        .collect();
    set.len() >= 2
}

#[test]
fn next_password_test() {
    assert_eq!(next_pwd("aaa"), "aab");
    assert_eq!(next_pwd("abc"), "abd");
    assert_eq!(next_pwd("xyz"), "xza");
    assert_eq!(next_pwd("xyzzz"), "xzaaa");
    assert_eq!(next_pwd("zzz"), "aaaa")
}
#[test]
fn requirement_1_examples() {
    assert!(requirement_1("hijklmmn"));
    assert!(!requirement_1("abbceffg"));
    assert!(!requirement_1("abbcegjk"));
}

#[test]
fn requirement_2_examples() {
    assert!(!requirement_2("hijklmmn"));
    assert!(requirement_2("abbceffg"));
    assert!(requirement_2("abbcegjk"));
}

#[test]
fn requirement_3_examples() {
    assert!(!requirement_3("hijklmmn"));
    assert!(requirement_3("abbceffg"));
    assert!(!requirement_3("abbcegjk"));
}

#[test]
fn part_one_example_one() {
    assert_eq!(next_password("abcdefgh"), "abcdffaa");
}

#[test]
fn part_one_example_two() {
    assert_eq!(next_password("ghijklmn"), "ghjaabcc");
}
