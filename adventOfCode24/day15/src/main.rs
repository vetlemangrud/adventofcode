use std::{collections::HashMap, fs, usize};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let map_input = input.split("\n\n").next().unwrap();
    let mut map = Vec::new();
    let mut robot: (i32, i32) = (0, 0);
    for (y, line) in map_input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(match c {
                '@' => {
                    robot = (x as i32, y as i32);
                    '@'
                }
                _ => c,
            });
        }
        map.push(row);
    }
    let height = map.len();
    let width = map[0].len();
    let move_input: Vec<_> = input
        .split("\n\n")
        .skip(1)
        .next()
        .unwrap()
        .lines()
        .flat_map(|l| l.chars())
        .collect();
    let directions: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (1, 0)), ('v', (0, 1)), ('<', (-1, 0)), ('^', (0, -1))]);
    for m in move_input.iter().map(|c| directions.get(&c).unwrap()) {
        let mut probe = (robot.0 + m.0, robot.1 + m.1);
        while map[probe.1 as usize][probe.0 as usize] == 'O' {
            probe = (probe.0 + m.0, probe.1 + m.1);
        }
        if map[probe.1 as usize][probe.0 as usize] == '#' {
            continue;
        }
        map[probe.1 as usize][probe.0 as usize] = 'O';
        map[(robot.1 + m.1) as usize][(robot.0 + m.0) as usize] = '@';
        map[robot.1 as usize][robot.0 as usize] = '.';
        robot = (robot.0 + m.0, robot.1 + m.1);
        // for y in 0..height {
        //     for x in 0..width {
        //         print!("{}", map[y][x]);
        //     }
        //     println!("");
        // }
        // println!("");
    }
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 'O' {
                total += y * 100 + x;
            }
        }
    }
    dbg!(total);
}
fn part2(input: String) {
    let map_input = input.split("\n\n").next().unwrap();
    let mut map = Vec::new();
    let mut robot: (i32, i32) = (0, 0);
    for (y, line) in map_input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line
            .chars()
            .flat_map(|c| match c {
                '#' => ['#', '#'],
                '@' => ['@', '.'],
                'O' => ['[', ']'],
                _ => ['.', '.'],
            })
            .enumerate()
        {
            row.push(match c {
                '@' => {
                    robot = (x as i32, y as i32);
                    '@'
                }
                _ => c,
            });
        }
        map.push(row);
    }
    let height = map.len();
    let width = map[0].len();
    let move_input: Vec<_> = input
        .split("\n\n")
        .skip(1)
        .next()
        .unwrap()
        .lines()
        .flat_map(|l| l.chars())
        .collect();
    let directions: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (1, 0)), ('v', (0, 1)), ('<', (-1, 0)), ('^', (0, -1))]);
    'm: for m in move_input.iter().map(|c| directions.get(&c).unwrap()) {
        let mut to_move = vec![('@', robot)];
        let mut border = vec![robot];
        while border.len() > 0 {
            let mut next_border = Vec::new();
            for (x, y) in &border {
                let ncoord = (x + m.0, y + m.1);
                let nchar = map[ncoord.1 as usize][ncoord.0 as usize];
                if nchar == '#' {
                    continue 'm;
                }
                if (nchar == '[' || nchar == ']') && m.1 == 0 {
                    next_border.push(ncoord);
                } else if nchar == '[' {
                    next_border.push(ncoord);
                    next_border.push((ncoord.0 + 1, ncoord.1));
                } else if nchar == ']' {
                    next_border.push(ncoord);
                    next_border.push((ncoord.0 - 1, ncoord.1));
                }
            }
            to_move.append(
                &mut next_border
                    .iter()
                    .copied()
                    .map(|(x, y)| (map[y as usize][x as usize], (x, y)))
                    .collect(),
            );
            border = next_border;
        }
        to_move.sort_by_key(|(_, (x, _))| -m.0 * x);
        to_move.sort_by_key(|(_, (_, y))| -m.1 * y);
        for (c, (x, y)) in to_move {
            if c == '@' {
                robot = (x + m.0, y + m.1);
            }
            map[y as usize][x as usize] = '.';
            map[(y + m.1) as usize][(x + m.0) as usize] = c;
        }

        // for y in 0..height {
        //     for x in 0..width {
        //         print!("{}", map[y][x]);
        //     }
        //     println!("");
        // }
        // println!("");
    }
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '[' {
                total += y * 100 + x;
            }
        }
    }
    dbg!(total);
}
