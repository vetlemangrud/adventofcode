use regex::Regex;
use std::{collections::HashSet, fs};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part1(input);
}
fn part1(input: String) {
    let mut lines = input.lines();
    let mut shapes: Vec<i64> = Vec::new();
    for _ in 0..6 {
        let mut count = 0;
        lines.next();
        for _ in 0..3 {
            let line = lines.next().unwrap();
            count += line.chars().filter(|c| c == &'#').count() as i64;
        }
        shapes.push(count);
        lines.next();
    }

    let line_regex = Regex::new(r"(\d+)x(\d+): (\d+) (\d+) (\d+) (\d+) (\d+) (\d+)").unwrap();

    let mut possible = 0;
    for line in lines {
        let captures = line_regex.captures(line).unwrap();

        let mut numbers = captures
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse::<i64>().unwrap());
        let width = numbers.next().unwrap();
        let height = numbers.next().unwrap();

        let mut max_tiles = width * height;
        for i in 0..6 {
            max_tiles -= shapes.get(i).unwrap() * numbers.next().unwrap();
        }
        if max_tiles >= 0 {
            possible += 1
        }
    }
    dbg!(possible);
}
