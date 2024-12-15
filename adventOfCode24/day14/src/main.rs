use std::{
    collections::HashSet,
    fs,
    io::{stdin, stdout, Write},
    usize,
};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (width, height) = (101, 103);
    part2(input, width, height);
}
fn part1(input: String, width: i32, height: i32) {
    let re = Regex::new(r"(-?\d+).+?(-?\d+).+?(-?\d+).+?(-?\d+)").unwrap();
    let robots: Vec<(i32, i32, i32, i32)> = re
        .captures_iter(&input)
        .map(|c| {
            (
                c.get(1).unwrap().as_str().parse().unwrap(),
                c.get(2).unwrap().as_str().parse().unwrap(),
                c.get(3).unwrap().as_str().parse().unwrap(),
                c.get(4).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();
    let robots_end: Vec<_> = robots
        .iter()
        .copied()
        .map(|(x, y, dx, dy)| {
            (
                (x + width * 100 + dx * 100) % width,
                (y + height * 100 + dy * 100) % height,
            )
        })
        .collect();
    let security: i32 = [(0, 0), (0, 1), (1, 0), (1, 1)]
        .iter()
        .map(|q| {
            robots_end
                .iter()
                .filter(|(x, _)| {
                    *x >= q.0 * (width + 1) / 2 && *x < (1 + q.0) * (width - 1) / 2 + q.0
                })
                .filter(|(_, y)| {
                    *y >= q.1 * (height + 1) / 2 && *y < (1 + q.1) * (height - 1) / 2 + q.1
                })
                .count() as i32
        })
        .product();
    dbg!(security);
}
fn part2(input: String, width: i32, height: i32) {
    let re = Regex::new(r"(-?\d+).+?(-?\d+).+?(-?\d+).+?(-?\d+)").unwrap();
    let robots: Vec<(i32, i32, i32, i32)> = re
        .captures_iter(&input)
        .map(|c| {
            (
                c.get(1).unwrap().as_str().parse().unwrap(),
                c.get(2).unwrap().as_str().parse().unwrap(),
                c.get(3).unwrap().as_str().parse().unwrap(),
                c.get(4).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();
    let mut i = 35;
    loop {
        let robots_end: Vec<_> = robots
            .iter()
            .copied()
            .map(|(x, y, dx, dy)| {
                (
                    (x + width * i + dx * i) % width,
                    (y + height * i + dy * i) % height,
                )
            })
            .collect();
        println!("Does this look like a christmas tree? (\"y/[n]\" or \"b\" to go backwards)");
        for y in 0..height {
            for x in 0..width {
                if robots_end.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        let mut s = String::new();
        stdout().flush();
        stdin().read_line(&mut s);
        if s == "y\n" {
            dbg!(i);
            break;
        } else if s == "b\n" {
            i -= 2 * width;
        }
        i += width;
    }
}
