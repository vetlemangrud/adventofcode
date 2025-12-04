use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::zip,
};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn neighbors(cell: (i32, i32), width: i32, height: i32) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for y_del in -1..=1 {
        for x_del in -1..=1 {
            let y = cell.1 + y_del;
            let x = cell.0 + x_del;

            if y < 0 || x < 0 {
                continue;
            }
            if y >= height || x >= width {
                continue;
            }
            if y == cell.1 && x == cell.0 {
                continue;
            }
            out.push((x, y));
        }
    }
    return out;
}

fn part1(input: String) {
    let mut map = HashMap::<(i32, i32), char>::new();
    let mut y = 0;
    let mut x = 0;
    for line in input.lines() {
        x = 0;
        for c in line.chars() {
            map.insert((x, y), c);
            x += 1;
        }
        y += 1;
    }
    let width = x;
    let height = y;
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            if map.get(&(x, y)).unwrap() == &'.' {
                continue;
            }
            let mut rolls = 0;
            for neighbor in neighbors((x, y), width, height) {
                if map.get(&neighbor).unwrap() == &'@' {
                    rolls += 1;
                }
            }
            if rolls < 4 {
                total += 1;
            }
        }
    }
    dbg!(total);
}

fn part2(input: String) {
    let mut map = HashMap::<(i32, i32), char>::new();
    let mut y = 0;
    let mut x = 0;
    for line in input.lines() {
        x = 0;
        for c in line.chars() {
            map.insert((x, y), c);
            x += 1;
        }
        y += 1;
    }
    let width = x;
    let height = y;
    let mut total = 0;
    loop {
        let mut has_removed = false;
        for y in 0..height {
            for x in 0..width {
                if map.get(&(x, y)).unwrap() == &'.' {
                    continue;
                }
                let mut rolls = 0;
                for neighbor in neighbors((x, y), width, height) {
                    if map.get(&neighbor).unwrap() == &'@' {
                        rolls += 1;
                    }
                }
                if rolls < 4 {
                    map.insert((x, y), '.');
                    total += 1;
                    has_removed = true;
                }
            }
        }
        if !has_removed {
            break;
        }
    }
    dbg!(total);
}
