pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn puzzle_one(input: &str) -> Result<String> {
    Ok(String::from(input))
}

pub fn puzzle_two(input: &str) -> Result<String> {
    Ok(String::from(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_one_example() -> Result<()> {
        assert_eq!(String::from("foo"), puzzle_one("foo")?);
        Ok(())
    }

    #[test]
    fn puzzle_two_example() -> Result<()> {
        assert_eq!(String::from("bar"), puzzle_two("bar")?);
        Ok(())
    }
}