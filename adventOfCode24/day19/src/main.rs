use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut valid: HashMap<&str, bool> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.trim())
        .map(|n| (n, true))
        .collect();
    let mut total = 0;
    let amount = input.lines().skip(2).count();
    let mut counted = 0;
    for pattern in input.lines().skip(2) {
        if is_valid(pattern, &mut valid) {
            total += 1;
        }
        counted += 1;
        println!("{}/{}", counted, amount)
    }
    dbg!(total);
}
fn is_valid<'a>(pattern: &'a str, valid: &mut HashMap<&'a str, bool>) -> bool {
    if pattern.len() == 0 {
        false
    } else if valid.contains_key(pattern) {
        *valid.get(pattern).unwrap()
    } else {
        let v = valid
            .clone()
            .iter()
            .filter(|(_, b)| **b)
            .map(|(s, _)| *s)
            .filter(|s| pattern.starts_with(*s))
            .any(|s| is_valid(&pattern[s.len()..], valid));
        valid.insert(pattern, v);
        v
    }
}

fn part2(input: String) {
    let axioms: Vec<_> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.trim())
        .collect();

    let mut total = 0;
    let mut teorems = HashMap::new();
    for pattern in input.lines().skip(2) {
        let t = options(pattern, &mut teorems, &axioms);
        total += t;
        println!("{} - {}", pattern, t)
    }
    dbg!(total);
}
fn options<'a>(pattern: &'a str, teorems: &mut HashMap<&'a str, i64>, axioms: &Vec<&str>) -> i64 {
    if pattern.len() == 0 {
        1
    } else if teorems.contains_key(pattern) {
        *teorems.get(pattern).unwrap()
    } else {
        let mut count = 0;
        for axiom in axioms.clone().iter().filter(|s| pattern.starts_with(*s)) {
            count += options(&pattern[axiom.len()..], teorems, axioms);
        }
        teorems.insert(pattern, count);
        count
    }
}
