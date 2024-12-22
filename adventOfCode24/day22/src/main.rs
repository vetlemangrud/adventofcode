use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut total = 0;
    for line in input.lines() {
        let mut secret_number = line.parse().unwrap();
        for _ in 0..2000 {
            secret_number = secret_number ^ (secret_number * 64);
            secret_number = prune(secret_number);
            secret_number = secret_number ^ (secret_number / 32);
            secret_number = prune(secret_number);
            secret_number = secret_number ^ (secret_number * 2048);
            secret_number = prune(secret_number);
        }
        total += secret_number;
    }
    dbg!(total);
}
fn part2(input: String) {
    let mut bananas: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    for line in input.lines() {
        let mut secret_number = line.parse().unwrap();
        let mut secret_numbers = Vec::from([secret_number]);
        for _ in 0..2000 {
            secret_number = secret_number ^ (secret_number * 64);
            secret_number = prune(secret_number);
            secret_number = secret_number ^ (secret_number / 32);
            secret_number = prune(secret_number);
            secret_number = secret_number ^ (secret_number * 2048);
            secret_number = prune(secret_number);
            secret_numbers.push(secret_number);
        }
        let mut seen = HashSet::new();
        for i in 0..secret_numbers.len() - 4 {
            let sn: Vec<i64> = secret_numbers.iter().map(|n| n % 10).collect();
            let pattern = (
                sn[i + 1] - sn[i],
                sn[i + 2] - sn[i + 1],
                sn[i + 3] - sn[i + 2],
                sn[i + 4] - sn[i + 3],
            );
            if !seen.contains(&pattern) {
                let value = sn[i + 4];
                let oldvalue = bananas.get(&pattern).unwrap_or(&0);
                bananas.insert(pattern, oldvalue + value);
                seen.insert(pattern);
            }
        }
    }
    dbg!(bananas.values().max());
}
fn prune(secret: i64) -> i64 {
    secret % 16777216
}
