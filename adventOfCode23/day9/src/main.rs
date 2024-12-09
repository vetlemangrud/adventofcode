use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut total = 0;
    for line in input.lines() {
        let values: Vec<i64> = line.split(" ").map(|n| n.parse().unwrap()).collect();
        total += next_value(values);
    }
    dbg!(total);
}

fn next_value(values: Vec<i64>) -> i64 {
    if values.iter().all(|v| *v == 0) {
        return 0;
    }
    let diffs: Vec<i64> = values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();
    return values.last().unwrap() + next_value(diffs);
}

fn part2(input: String) {
    let mut total = 0;
    for line in input.lines() {
        let values: Vec<i64> = line.split(" ").map(|n| n.parse().unwrap()).collect();
        total += next_value(values.iter().rev().map(|v| *v).collect());
    }
    dbg!(total);
}
