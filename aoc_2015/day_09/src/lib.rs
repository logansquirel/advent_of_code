use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> u32 {
    let map = init(input);
    let cities: HashSet<_> = map.keys().flat_map(|k| k.iter().copied()).collect();
    let paths = permutations(cities);
    let mut distances = Vec::new();
    for path in paths {
        let distance = distance(path, &map);
        distances.push(distance);
    }
    distances.into_iter().min().unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let map = init(input);
    let cities: HashSet<_> = map.keys().flat_map(|k| k.iter().copied()).collect();
    let paths = permutations(cities);
    let mut distances = Vec::new();
    for path in paths {
        let distance = distance(path, &map);
        distances.push(distance);
    }
    distances.into_iter().max().unwrap()
}

fn init(input: &str) -> HashMap<[&str; 2], u32> {
    let mut map = HashMap::new();
    let regex = regex::Regex::new(r"^([A-Za-z]+) to ([A-Za-z]+) = (\d+)$").unwrap();
    for line in input.trim().lines() {
        let caps = regex
            .captures(line)
            .unwrap_or_else(|| panic!("invalid distance: '{}", line));
        let from = caps.get(1).unwrap().as_str();
        let to = caps.get(2).unwrap().as_str();
        let dist = caps.get(3).unwrap().as_str().parse().unwrap();
        map.insert([from, to], dist);
        map.insert([to, from], dist);
    }
    map
}

fn distance<'a>(path: Vec<&'a str>, map: &HashMap<[&'a str; 2], u32>) -> u32 {
    path.into_iter()
        .as_slice()
        .windows(2)
        .map(|route| map.get(route).unwrap())
        .sum()
}

fn permutations<'a, I>(col: I) -> Vec<Vec<&'a str>>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut set: Vec<_> = col.into_iter().collect();
    let mut permutations = Vec::new();
    let size = set.len();
    heap(size, &mut set, &mut permutations);
    permutations
}

fn heap<T: Clone>(size: usize, set: &mut Vec<T>, permutations: &mut Vec<Vec<T>>) {
    if size == 1 {
        permutations.push(set.clone())
    } else {
        heap(size - 1, set, permutations);
        for i in 0..size - 1 {
            if size % 2 == 0 {
                set.swap(i, size - 1);
            } else {
                set.swap(0, size - 1);
            }
            heap(size - 1, set, permutations);
        }
    }
}

#[test]
fn part_one_example() {
    let input = r#"
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
"#;

    assert_eq!(part_one(input), 605)
}

#[test]
fn part_two_example() {
    let input = r#"
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
"#;

    assert_eq!(part_two(input), 982)
}
