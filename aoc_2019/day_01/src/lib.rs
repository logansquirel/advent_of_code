pub fn part_one(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter_map(|l| l.parse::<u32>().ok())
        .map(|x| x / 3 - 2)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let iter = input.trim().lines().filter_map(|l| l.parse::<u32>().ok());
    let mut total_fuel = 0;
    for module_mass in iter {
        let mut module_fuel = 0;
        let mut mass = (module_mass / 3 - 2) as i32;
        while mass > 0 {
            module_fuel += mass;
            mass = mass / 3 - 2;
        }
        total_fuel += module_fuel;
    }
    total_fuel as u32
}

#[test]
fn part_one_example_one() {
    assert_eq!(2, part_one("12"));
}

#[test]
fn part_one_example_two() {
    assert_eq!(2, part_one("14"));
}

#[test]
fn part_one_example_three() {
    assert_eq!(654, part_one("1969"));
}

#[test]
fn part_one_example_four() {
    assert_eq!(33583, part_one("100756"));
}

#[test]
fn part_two_example_one() {
    assert_eq!(2, part_two("14"));
}

#[test]
fn part_two_example_two() {
    assert_eq!(966, part_two("1969"));
}

#[test]
fn part_two_example_three() {
    assert_eq!(50346, part_two("100756"));
}
