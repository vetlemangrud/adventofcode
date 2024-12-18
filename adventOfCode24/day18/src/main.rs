use std::{
    collections::{HashMap, HashSet},
    fmt::write,
    fs,
};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input, 71);
}
fn part1(input: String, gridsize: i32, no_bytes: usize) {
    let bytepos: HashSet<(i32, i32)> = input
        .lines()
        .map(|l| l.split(",").map(|v| v.parse::<i32>().unwrap()).collect())
        .map(|v: Vec<i32>| (v[0], v[1]))
        .take(no_bytes)
        .collect();
    let mut visited = HashSet::new();
    let mut border = HashSet::from([(0, 0)]);
    let offsets = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut steps = 0;
    loop {
        for ele in border.clone() {
            visited.insert(ele);
        }
        let next_border = border
            .iter()
            .copied()
            .flat_map(|(x, y)| offsets.map(|(dx, dy)| (x + dx, y + dy)))
            .filter(|(x, _)| *x < gridsize && *x >= 0)
            .filter(|(_, y)| *y < gridsize && *y >= 0)
            .filter(|(x, y)| !visited.contains(&(*x, *y)))
            .filter(|(x, y)| !bytepos.contains(&(*x, *y)))
            .collect();
        border = next_border;
        if visited.contains(&(gridsize - 1, gridsize - 1)) {
            break;
        }
        steps += 1;
    }
    dbg!(steps);
}
fn part2(input: String, gridsize: i32) {
    let offsets = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    'outer: for no_bytes in 1024..input.lines().count() {
        let bytepos: HashSet<(i32, i32)> = input
            .lines()
            .take(no_bytes)
            .map(|l| l.split(",").map(|v| v.parse::<i32>().unwrap()).collect())
            .map(|v: Vec<i32>| (v[0], v[1]))
            .collect();
        let mut visited = HashSet::new();
        let mut border = HashSet::from([(0, 0)]);

        loop {
            for ele in border.clone() {
                visited.insert(ele);
            }
            let next_border = border
                .iter()
                .copied()
                .flat_map(|(x, y)| offsets.map(|(dx, dy)| (x + dx, y + dy)))
                .filter(|(x, _)| *x < gridsize && *x >= 0)
                .filter(|(_, y)| *y < gridsize && *y >= 0)
                .filter(|(x, y)| !visited.contains(&(*x, *y)))
                .filter(|(x, y)| !bytepos.contains(&(*x, *y)))
                .collect();
            border = next_border;
            if visited.contains(&(gridsize - 1, gridsize - 1)) {
                break;
            }
            if border.len() == 0 {
                dbg!(input.lines().nth(no_bytes - 1));
                break 'outer;
            }
        }
    }
}
