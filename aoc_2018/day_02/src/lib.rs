use std::collections::HashMap;

pub fn puzzle_one(input: &str) -> u32 {
    let mut twice = 0;
    let mut three = 0;
    for line in input.trim().lines() {
        let mut seen = HashMap::new();
        for char in line.trim().chars() {
            seen.entry(char).and_modify(|v| *v += 1).or_insert(1);
        }
        if seen.values().any(|&v| v == 2) {
            twice += 1;
        }
        if seen.values().any(|&v| v == 3) {
            three += 1;
        }
    }
    twice * three
}

pub fn puzzle_two(input: &str) -> String {
    let mut common = String::new();
    for (line, id1) in input.trim().lines().enumerate() {
        for id2 in input.trim().lines().skip(line) {
            match compare_box_ids(id1, id2) {
                Some(idx) => common = format!("{}{}", &id1[..idx], &id1[(idx + 1)..]),
                None => continue,
            }
        }
    }
    common
}

fn compare_box_ids(id1: &str, id2: &str) -> Option<usize> {
    let mut pos = None;
    for (i, (c1, c2)) in id1.chars().zip(id2.chars()).enumerate() {
        if c1 != c2 {
            match pos {
                Some(_) => return None,
                None => pos = Some(i),
            }
        }
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        assert_eq!(
            12,
            puzzle_one("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab")
        );
    }

    #[test]
    fn puzzle_two_example() {
        assert_eq!(
            String::from("fgij"),
            puzzle_two("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz")
        );
    }
}
