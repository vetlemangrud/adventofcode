use std::collections::HashSet;
use std::fs;

fn priority(line: &str) -> u32 {
    let line: Vec<char> = line.chars().collect();
    let p = line.len() / 2;
    let common: char = **HashSet::<&char>::from_iter(line[..p].iter())
        .intersection(&HashSet::<&char>::from_iter(line[p..].iter()))
        .next()
        .unwrap_or(&&'0');

    let val = common as u32;
    if val >= 97 {
        val - 96
    } else if val >= 65 {
        val - 64 + 26
    } else {
        0
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut lines: Vec<&str> = input.split("\n").collect();
    lines.pop();
    part2(&lines);
    let priorities: Vec<u32> = lines.iter().map(|line| priority(line)).collect();
    let priority_sum: u32 = priorities.iter().sum();
    dbg!(priority_sum);
}

fn part2(lines: &Vec<&str>) {
    let mut sum = 0;
    for group in lines.chunks(3) {
        dbg!(group);
        let sets = group
            .iter()
            .map(|line| HashSet::<char>::from_iter(line.chars()))
            .collect::<Vec<_>>();
        let set1 = sets.get(0).unwrap();
        let set2 = sets.get(1).unwrap();
        let set3 = sets.get(2).unwrap();
        let intersection = (&(set1 & set2) & set3);
        let badge = intersection.iter().next().unwrap();
        dbg!(badge);
        let mut val = *badge as u32;
        if val >= 97 {
            val -= 96
        } else if val >= 65 {
            val = val - 64 + 26
        } else {
            val = 0
        }
        sum += val;
        dbg!(val);
    }
    dbg!(sum);
}
