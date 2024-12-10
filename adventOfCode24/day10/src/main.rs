use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut levels = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            levels.insert((x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }
    let mut trails: HashSet<_> = levels
        .iter()
        .filter(|(_, h)| **h == 0)
        .map(|((x, y), _)| ((*x, *y), (*x, *y)))
        .collect();
    let offsets = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    for next_h in 1..=9 {
        let next_trails: HashSet<_> = trails
            .iter()
            .flat_map(|(s, (x, y))| offsets.iter().map(|(dx, dy)| (*s, (*x + *dx, *y + *dy))))
            .filter(|(_, (x, y))| *levels.get(&(*x, *y)).unwrap_or(&999) == next_h)
            .collect();
        trails = next_trails;
    }
    dbg!(trails.len());
}

fn part2(input: String) {
    let mut levels = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            levels.insert((x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }
    let mut trails: Vec<_> = levels
        .iter()
        .filter(|(_, h)| **h == 0)
        .map(|((x, y), _)| ((*x, *y), (*x, *y)))
        .collect();
    let offsets = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    for next_h in 1..=9 {
        let next_trails: Vec<_> = trails
            .iter()
            .flat_map(|(s, (x, y))| offsets.iter().map(|(dx, dy)| (*s, (*x + *dx, *y + *dy))))
            .filter(|(_, (x, y))| *levels.get(&(*x, *y)).unwrap_or(&999) == next_h)
            .collect();
        trails = next_trails;
    }
    dbg!(trails.len());
}
