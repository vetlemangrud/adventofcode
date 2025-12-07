use std::{
    collections::{HashMap, HashSet},
    fs::{self},
};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut lines = input.lines();
    let mut laser_positions: HashSet<usize> = HashSet::new();
    let mut split_count = 0;
    laser_positions.insert(lines.next().unwrap().find('S').unwrap());
    for line in lines {
        let prev_postions = laser_positions.clone();
        for (i, c) in line.chars().enumerate() {
            if c == '^' && prev_postions.contains(&i) {
                split_count += 1;
                laser_positions.remove(&i);
                laser_positions.insert(i - 1);
                laser_positions.insert(i + 1);
            }
        }
    }
    dbg!(split_count);
}

fn part2(input: String) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut laser_worlds: HashMap<usize, i64> = HashMap::new();
    let initial_pos = first_line.find('S').unwrap();
    laser_worlds.insert(initial_pos, 1);
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '^' {
                let num_worlds = laser_worlds.remove(&i).unwrap_or(0);
                let prev_left = *laser_worlds.get(&(i - 1)).unwrap_or(&0);
                let prev_right = *laser_worlds.get(&(i + 1)).unwrap_or(&0);
                laser_worlds.insert(i - 1, prev_left + num_worlds);
                laser_worlds.insert(i + 1, prev_right + num_worlds);
            }
        }
    }
    let total: i64 = laser_worlds.values().sum();
    dbg!(total);
}
