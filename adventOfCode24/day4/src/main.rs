use std::{collections::HashMap, fs};

use itertools::Itertools;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn part1(input: String) {
    let mut input_map: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            input_map.insert((x as i32, y as i32), char);
        }
    }
    let word = "XMAS";
    let offsets: Vec<(i32, i32)> = vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut matches = 0;
    dbg!((0..140).cartesian_product(0..140).count());
    for start_pos in (0..140).cartesian_product(0..140) {
        dbg!(start_pos);
        'outer: for offset in &offsets {
            for i in 0..word.len() {
                if *input_map
                    .get(&(
                        start_pos.0 + i as i32 * offset.0,
                        start_pos.1 + i as i32 * offset.1,
                    ))
                    .unwrap_or(&'?')
                    != word.chars().nth(i).unwrap()
                {
                    continue 'outer;
                }
            }
            matches += 1;
        }
    }
    dbg!(matches);
}

fn part2(input: String) {
    let mut input_map: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            input_map.insert((x as i32, y as i32), char);
        }
    }
    let mut matches = 0;
    let offsets = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)];
    dbg!((0..140).cartesian_product(0..140).count());
    for start_pos in (0..140).cartesian_product(0..140) {
        if input_map.get(&start_pos).unwrap_or(&'?') != &'A' {
            continue;
        }
        let mut neighbors = String::new();
        for offset in &offsets {
            let c = input_map
                .get(&(start_pos.0 + offset.0, start_pos.1 + offset.1))
                .unwrap_or(&'?');
            neighbors.push(*c);
        }
        dbg!(&neighbors);
        let match_neighbors = vec!["MMSS", "MSSM", "SSMM", "SMMS"];
        for m in match_neighbors {
            if neighbors == m {
                matches += 1
            }
        }
    }
    dbg!(matches);
}
