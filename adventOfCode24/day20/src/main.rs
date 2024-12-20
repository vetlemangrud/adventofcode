use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let save_lim = 100;
    part2(input, save_lim);
}
fn part1(input: String) {
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut walls = Vec::new();
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'S' {
                start = (x as i32, y as i32);
            }
            if c == 'E' {
                end = (x as i32, y as i32);
            }
            if c == '#' {
                walls.push((x as i32, y as i32));
            }
        }
        map.push(row);
    }
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let times: Vec<_> = walls
        .iter()
        .filter(|(x, _)| *x != 0)
        .filter(|(_, y)| *y != 0)
        .filter(|(x, _)| *x < width - 1)
        .filter(|(_, y)| *y < height - 1)
        .map(|skipped| {
            let mut visited: HashSet<(i32, i32)> = HashSet::from_iter(walls.iter().cloned());
            visited.remove(skipped);
            let mut border = Vec::from([start]);
            let mut steps = 0;
            loop {
                if border.contains(&end) {
                    break;
                }
                let mut new_border = Vec::new();
                for (x, y) in &border {
                    visited.insert((*x, *y));
                    for (dx, dy) in directions {
                        let next = (x + dx, y + dy);

                        if visited.contains(&next) {
                            continue;
                        }
                        new_border.push(next);
                    }
                }
                border = new_border;
                steps += 1;
            }
            steps
        })
        .collect();
    let longest = times.iter().copied().max().unwrap();
    dbg!(times.iter().copied().filter(|t| t + 100 <= longest).count());
}

fn part2(input: String, save_lim: usize) {
    let mut map = Vec::new();
    let mut start = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'S' {
                start = (x as i32, y as i32);
            }
        }
        map.push(row);
    }
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut path = Vec::new();
    let mut current = start;
    loop {
        let next = directions
            .iter()
            .map(|(dx, dy)| (current.0 + dx, current.1 + dy))
            .filter(|(x, y)| !path.contains(&(*x, *y)))
            .filter(|(x, y)| map[*y as usize][*x as usize] != '#')
            .next();
        path.push(current);
        match next {
            Some(n) => {
                current = n;
            }
            None => {
                break;
            }
        }
    }
    dbg!(path.len());

    let mut valid_cheats = 0;
    for start_i in 0..path.len() - save_lim {
        for end_i in start_i + save_lim..path.len() {
            let (sx, sy) = path[start_i];
            let (ex, ey) = path[end_i];
            let dist = (ex - sx).abs() + (ey - sy).abs();
            if (dist) <= 20 && start_i + dist as usize + save_lim <= end_i {
                valid_cheats += 1;
            }
        }
    }
    dbg!(valid_cheats);
    // Example has 285
}
