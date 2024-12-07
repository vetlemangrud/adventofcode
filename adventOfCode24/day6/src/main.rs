use std::{collections::HashSet, fs};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn part1(input: String) {
    let mut walls: HashSet<(i32, i32)> = HashSet::new();
    let mut guard = (0, 0);
    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut guard_d = 0;
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                walls.insert((x as i32, y as i32));
            } else if char == '^' {
                guard = (x as i32, y as i32);
            }
        }
    }

    while guard.0 >= 0 && guard.0 < width && guard.1 >= 0 && guard.1 < height {
        let offset = directions.get(guard_d).unwrap();
        let next_pos = (guard.0 + offset.0, guard.1 + offset.1);
        if walls.contains(&next_pos) {
            guard_d += 1;
            guard_d %= 4;
        } else {
            visited.insert(guard);
            guard = next_pos;
        }
    }
    dbg!(visited.iter().count());
}
fn part2(input: String) {
    let mut og_walls: HashSet<(i32, i32)> = HashSet::new();
    let mut og_guard = (0, 0);
    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut visited: HashSet<((i32, i32), usize)> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                og_walls.insert((x as i32, y as i32));
            } else if char == '^' {
                og_guard = (x as i32, y as i32);
            }
        }
    }

    let mut loops = 0;
    for x in 0..width {
        for y in 0..height {
            let new_wall = (x as i32, y as i32);
            if new_wall == og_guard || og_walls.contains(&new_wall) {
                continue;
            }
            let mut guard = og_guard.clone();
            let mut walls = og_walls.clone();
            let mut guard_d = 0;
            visited.clear();
            walls.insert(new_wall);
            while guard.0 >= 0 && guard.0 < width && guard.1 >= 0 && guard.1 < height {
                let offset = directions.get(guard_d).unwrap();
                let next_pos = (guard.0 + offset.0, guard.1 + offset.1);
                if walls.contains(&next_pos) {
                    guard_d += 1;
                    guard_d %= 4;
                } else {
                    if visited.contains(&(guard, guard_d)) {
                        loops += 1;
                        break;
                    }

                    visited.insert((guard, guard_d));
                    guard = next_pos;
                }
            }
        }
    }
    dbg!(loops);
}
