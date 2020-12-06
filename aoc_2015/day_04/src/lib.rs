use md5::Digest;
use std::io::Write;

pub fn part_one(input: &str) -> u32 {
    let mut hasher = md5::Md5::new();
    let key = input.trim().as_bytes();
    let mut int = 0;
    let mut buf: Vec<u8> = Vec::with_capacity(10);
    loop {
        int += 1;
        write!(&mut buf, "{}", int).unwrap();
        hasher.update(key);
        hasher.update(&buf);
        let hash = hasher.finalize_reset();
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            break int;
        }
        buf.clear();
    }
}

pub fn part_two(input: &str) -> u32 {
    let mut hasher = md5::Md5::new();
    let key = input.trim().as_bytes();
    let mut int = 0;
    let mut buf: Vec<u8> = Vec::with_capacity(10);
    loop {
        int += 1;
        write!(&mut buf, "{}", int).unwrap();
        hasher.update(key);
        hasher.update(&buf);
        let hash = hasher.finalize_reset();
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            break int;
        }
        buf.clear();
    }
}

#[test]
fn part_one_example_one() {
    assert_eq!(609043, part_one("abcdef"));
}

#[test]
fn part_one_example_two() {
    assert_eq!(1048970, part_one("pqrstuv"));
}
