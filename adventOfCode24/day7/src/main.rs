use std::{collections::HashSet, fs, hash::Hash};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let total = input.lines().fold(0, |total, line| {
        let test_value: i64 = line.split(":").next().unwrap().parse().unwrap();
        let possible_values = line
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(" ")
            .map(|v| v.parse().unwrap())
            .fold(HashSet::new(), |possible, new: i64| {
                if possible.is_empty() {
                    return HashSet::from([new]);
                }
                possible.iter().flat_map(|v| [v + new, v * new]).collect()
            });
        total
            + if possible_values.contains(&test_value) {
                test_value
            } else {
                0
            }
    });
    dbg!(total);
}
fn new_op(a: i64, b: i64) -> i64 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}
fn part2(input: String) {
    let total = input.lines().fold(0, |total, line| {
        let test_value: i64 = line.split(":").next().unwrap().parse().unwrap();
        let possible_values = line
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(" ")
            .map(|v| v.parse().unwrap())
            .fold(HashSet::new(), |possible, new: i64| {
                if possible.is_empty() {
                    return HashSet::from([new]);
                }
                possible
                    .iter()
                    .flat_map(|v| [v + new, v * new, new_op(*v, new)])
                    .collect()
            });
        total
            + if possible_values.contains(&test_value) {
                test_value
            } else {
                0
            }
    });
    dbg!(total);
}
