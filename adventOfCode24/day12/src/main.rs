use std::{collections::HashSet, fs, usize};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push((char, false));
        }
        map.push(row);
    }
    let height = map.len();
    let width = map[0].len();
    let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x].1 {
                continue;
            }
            map[y][x] = (map[y][x].0, true);
            let mut border = Vec::from([(y, x)]);
            let mut fences = 0;
            let mut area = 1;
            while border.len() > 0 {
                let (by, bx) = border.pop().unwrap();
                let b_spot = map[by][bx];
                for (ney, nex) in directions.map(|d| (by as i32 + d.0, bx as i32 + d.1)) {
                    let neighbor = map.get(ney as usize).map(|r| r.get(nex as usize));
                    if neighbor.is_none()
                        || neighbor.unwrap().is_none()
                        || neighbor.unwrap().unwrap().0 != b_spot.0
                    {
                        fences += 1;
                    } else if !neighbor.unwrap().unwrap().1 {
                        map[ney as usize][nex as usize] = (map[ney as usize][nex as usize].0, true);
                        border.push((ney as usize, nex as usize));
                        area += 1;
                    }
                }
            }
            total += area * fences;
        }
    }
    dbg!(total);
}

fn part2(input: String) {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push((char, false));
        }
        map.push(row);
    }
    let height = map.len();
    let width = map[0].len();
    let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x].1 {
                continue;
            }
            map[y][x] = (map[y][x].0, true);
            let mut border = Vec::from([(y, x)]);
            let mut shape = HashSet::from([(y as i32, x as i32)]);
            let shape_c = map[y][x].0;
            while border.len() > 0 {
                let (by, bx) = border.pop().unwrap();
                let b_spot = map[by][bx];
                for (ney, nex) in directions.map(|d| (by as i32 + d.0, bx as i32 + d.1)) {
                    let neighbor = map.get(ney as usize).map(|r| r.get(nex as usize));
                    if neighbor.is_none()
                        || neighbor.unwrap().is_none()
                        || neighbor.unwrap().unwrap().0 != b_spot.0
                    {
                    } else if !neighbor.unwrap().unwrap().1 {
                        map[ney as usize][nex as usize] = (map[ney as usize][nex as usize].0, true);
                        border.push((ney as usize, nex as usize));
                        shape.insert((ney, nex));
                    }
                }
            }

            let min_y = shape.iter().copied().map(|(y, _)| y).min().unwrap();
            let max_y = shape.iter().copied().map(|(y, _)| y).max().unwrap();
            let min_x = shape.iter().copied().map(|(_, x)| x).min().unwrap();
            let max_x = shape.iter().copied().map(|(_, x)| x).max().unwrap();
            let mut sides = 0;
            for y in min_y..=max_y {
                let mut prev = (false, false);
                for x in min_x..=max_x {
                    if shape.contains(&(y, x)) {
                        if y == 0 || !shape.contains(&(y - 1, x)) {
                            prev = (true, prev.1);
                        } else if prev.0 {
                            prev = (false, prev.1);
                            sides += 1;
                        }
                        if y + 1 == height as i32 || !shape.contains(&(y + 1, x)) {
                            prev = (prev.0, true);
                        } else if prev.1 {
                            prev = (prev.0, false);
                            sides += 1;
                        }
                    } else {
                        if prev.0 {
                            prev = (false, prev.1);
                            sides += 1;
                        }
                        if prev.1 {
                            prev = (prev.0, false);
                            sides += 1;
                        }
                    }
                }
                if prev.0 {
                    sides += 1;
                }
                if prev.1 {
                    sides += 1;
                }
            }
            for x in min_x..=max_x {
                let mut prev = (false, false);
                for y in min_y..=max_y {
                    if shape.contains(&(y, x)) {
                        if x == 0 || !shape.contains(&(y, x - 1)) {
                            prev = (true, prev.1);
                        } else if prev.0 {
                            prev = (false, prev.1);
                            sides += 1;
                        }
                        if x + 1 == width as i32 || !shape.contains(&(y, x + 1)) {
                            prev = (prev.0, true);
                        } else if prev.1 {
                            prev = (prev.0, false);
                            sides += 1;
                        }
                    } else {
                        if prev.0 {
                            prev = (false, prev.1);
                            sides += 1;
                        }
                        if prev.1 {
                            prev = (prev.0, false);
                            sides += 1;
                        }
                    }
                }
                if prev.0 {
                    sides += 1;
                }
                if prev.1 {
                    sides += 1;
                }
            }
            total += sides * shape.len();
        }
    }
    dbg!(total);
}
