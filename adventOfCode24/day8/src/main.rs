use std::{collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut antennas = HashSet::new();
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;
    dbg!(&(width, height));
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antennas.insert((x as i32, y as i32, char));
            }
        }
    }
    let antinodes: HashSet<(i32, i32)> = antennas
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| a != b)
        .filter(|(a, b)| a.2 == b.2)
        .flat_map(|(a, b)| {
            [
                (b.0 + b.0 - a.0, b.1 + b.1 - a.1),
                (a.0 + a.0 - b.0, a.1 + a.1 - b.1),
            ]
        })
        .filter(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
        .collect();
    dbg!(&antinodes.len());
}

fn part2(input: String) {
    let mut antennas = HashSet::new();
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;
    dbg!(&(width, height));
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antennas.insert((x as i32, y as i32, char));
            }
        }
    }

    let antinodes: HashSet<(i32, i32)> = antennas
        .iter()
        .cartesian_product(antennas.iter())
        .filter(|(a, b)| a != b)
        .filter(|(a, b)| a.2 == b.2)
        .flat_map(|(a, b)| {
            let mut delta = (b.0 - a.0, b.1 - a.1);
            let divisor =
                gcd::euclid_u32(i32::abs(delta.0) as u32, i32::abs(delta.1) as u32) as i32;
            delta = (delta.0 / divisor, delta.1 / divisor);
            let an: Vec<_> = (0..)
                .map(|m| (a.0 + m * delta.0, a.1 + m * delta.1))
                .take_while(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
                .collect();
            an
        })
        .collect();
    dbg!(&antinodes.len());
}
