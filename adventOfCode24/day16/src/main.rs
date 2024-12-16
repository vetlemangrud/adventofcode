use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) -> i32 {
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
    let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut considered: HashSet<((i32, i32), usize)> = HashSet::from([(start, 0)]);
    let mut border: Vec<((i32, i32), usize, i32)> = vec![(start, 0, 0)];
    while border.len() > 0 {
        border.sort_by_key(|(_, _, c)| -c);
        let next = border.pop().unwrap();
        if map[next.0 .1 as usize][next.0 .0 as usize] == 'E' {
            dbg!(next.2);
            return next.2;
        }
        if !considered.contains(&(next.0, (next.1 + 1) % 4)) {
            considered.insert((next.0, (next.1 + 1) % 4));
            border.push((next.0, (next.1 + 1) % 4, next.2 + 1000));
        }
        if !considered.contains(&(next.0, (next.1 + 3) % 4)) {
            considered.insert((next.0, (next.1 + 3) % 4));
            border.push((next.0, (next.1 + 3) % 4, next.2 + 1000));
        }
        if !considered.contains(&(
            (
                next.0 .0 + directions[next.1].0,
                next.0 .1 + directions[next.1].1,
            ),
            next.1,
        )) && map[(next.0 .1 + directions[next.1].1) as usize]
            [(next.0 .0 + directions[next.1].0) as usize]
            != '#'
        {
            considered.insert((
                (
                    next.0 .0 + directions[next.1].0,
                    next.0 .1 + directions[next.1].1,
                ),
                next.1,
            ));
            border.push((
                (
                    next.0 .0 + directions[next.1].0,
                    next.0 .1 + directions[next.1].1,
                ),
                next.1,
                next.2 + 1,
            ));
        }
    }
    return 0;
}

fn part2(input: String) {
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
    let target_score = part1(input);
    let mut spots = HashSet::new();
    let mut markmap: HashMap<(i32, i32, usize, usize, i32), bool> = HashMap::new();
    mark_paths(
        start.0,
        start.1,
        0,
        0,
        target_score,
        &map,
        &mut spots,
        &mut markmap,
    );
    let height = map.len();
    let width = map[0].len();
    for y in 0..height {
        for x in 0..width {
            if spots.contains(&(x as i32, y as i32)) {
                print!("O");
            } else {
                print!("{}", map[y as usize][x as usize]);
            }
        }
        println!("")
    }
    dbg!(spots.len());
}
fn mark_paths(
    x: i32,
    y: i32,
    dir: usize,
    last_dir: usize,
    score_left: i32,
    map: &Vec<Vec<char>>,
    spots: &mut HashSet<(i32, i32)>,
    markmap: &mut HashMap<(i32, i32, usize, usize, i32), bool>,
) -> bool {
    if markmap.contains_key(&(x, y, dir, last_dir, score_left)) {
        return *markmap.get(&(x, y, dir, last_dir, score_left)).unwrap();
    }
    if map[y as usize][x as usize] == 'E' {
        markmap.insert((x, y, dir, last_dir, score_left), true);
        spots.insert((x, y));
        return true;
    }

    let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut marked = false;
    let off = directions[dir];
    if score_left >= 1 && map[(y + off.1) as usize][(x + off.0) as usize] != '#' {
        marked = mark_paths(
            x + off.0,
            y + off.1,
            dir,
            dir,
            score_left - 1,
            map,
            spots,
            markmap,
        );
    }
    if score_left >= 1000 && dir == last_dir {
        marked = mark_paths(
            x,
            y,
            (dir + 1) % 4,
            dir,
            score_left - 1000,
            map,
            spots,
            markmap,
        ) || marked;
        marked = mark_paths(
            x,
            y,
            (dir + 3) % 4,
            dir,
            score_left - 1000,
            map,
            spots,
            markmap,
        ) || marked;
    }
    if marked {
        spots.insert((x, y));
    }
    markmap.insert((x, y, dir, last_dir, score_left), marked);
    marked
}
