use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> i32 {
    let table = init(input.trim());
    let people: HashSet<_> = table.keys().flat_map(|k| k.iter().copied()).collect();
    let seating = permutations(people);
    seating
        .into_iter()
        .map(|plan| happiness(plan, &table))
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> i32 {
    let mut table = init(input.trim());
    let mut people: HashSet<_> = table.keys().flat_map(|k| k.iter().copied()).collect();
    for person in people.iter() {
        table.insert(["Me", person], 0);
        table.insert([person, "Me"], 0);
    }
    people.insert("Me");
    let seating = permutations(people);
    seating
        .into_iter()
        .map(|plan| happiness(plan, &table))
        .max()
        .unwrap()
}

fn init(input: &str) -> HashMap<[&str; 2], i32> {
    let mut map = HashMap::new();
    for line in input.trim().lines() {
        let mut split = line
            .trim()
            .strip_suffix('.')
            .unwrap()
            .split_ascii_whitespace();
        let from = split.next().unwrap();
        let sign = match split.nth(1).unwrap() {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!("invalid input"),
        };
        let value = split.next().unwrap().parse::<i32>().unwrap();
        let happiness = sign * value;
        let to = split.nth(6).unwrap();
        map.insert([from, to], happiness);
    }
    map
}

fn happiness(plan: Vec<&str>, people: &HashMap<[&str; 2], i32>) -> i32 {
    let mut happiness = plan
        .iter()
        .as_slice()
        .windows(2)
        .map(|relation| {
            people.get(&[relation[0], relation[1]]).unwrap()
                + people.get(&[relation[1], relation[0]]).unwrap()
        })
        .sum();
    let len = plan.len();
    happiness += people.get(&[plan[0], plan[len - 1]]).unwrap()
        + people.get(&[plan[len - 1], plan[0]]).unwrap();
    happiness
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
    Alice would gain 54 happiness units by sitting next to Bob.
    Alice would lose 79 happiness units by sitting next to Carol.
    Alice would lose 2 happiness units by sitting next to David.
    Bob would gain 83 happiness units by sitting next to Alice.
    Bob would lose 7 happiness units by sitting next to Carol.
    Bob would lose 63 happiness units by sitting next to David.
    Carol would lose 62 happiness units by sitting next to Alice.
    Carol would gain 60 happiness units by sitting next to Bob.
    Carol would gain 55 happiness units by sitting next to David.
    David would gain 46 happiness units by sitting next to Alice.
    David would lose 7 happiness units by sitting next to Bob.
    David would gain 41 happiness units by sitting next to Carol.
    "#;
    assert_eq!(part_one(input), 330);
}
