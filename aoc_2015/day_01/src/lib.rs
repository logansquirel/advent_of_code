pub fn puzzle_one(input: &str) -> i32 {
    let mut floor = 0;
    for char in input.trim().chars() {
        match char {
            ')' => floor -= 1,
            '(' => floor += 1,
            x => panic!(
                "invalid instruction :'{}'",
                x.escape_default().collect::<String>()
            ),
        }
    }
    floor
}

pub fn puzzle_two(input: &str) -> usize {
    let mut floor = 0;
    for (pos, char) in input.trim().char_indices() {
        match char {
            ')' => floor -= 1,
            '(' => floor += 1,
            x => panic!(
                "invalid instruction :'{}'",
                x.escape_default().collect::<String>()
            ),
        }
        if floor == -1 {
            return pos + 1;
        }
    }
    panic!("basement not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_one_example_one() {
        assert_eq!(0, puzzle_one("(())"));
        assert_eq!(0, puzzle_one("()()"));
    }

    #[test]
    fn puzzle_one_example_two() {
        assert_eq!(3, puzzle_one("((("));
        assert_eq!(3, puzzle_one("(()(()("));
    }

    #[test]
    fn puzzle_one_example_three() {
        assert_eq!(3, puzzle_one("))((((("));
    }

    #[test]
    fn puzzle_one_example_four() {
        assert_eq!(-1, puzzle_one("())"));
        assert_eq!(-1, puzzle_one("))("));
    }

    #[test]
    fn puzzle_one_example_five() {
        assert_eq!(-3, puzzle_one(")))"));
        assert_eq!(-3, puzzle_one(")())())"));
    }
}
