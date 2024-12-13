use itertools::{self, Itertools};
use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let mut total = 0;
    for game in re.captures_iter(&input) {
        let ax: i32 = game.get(1).unwrap().as_str().parse().unwrap();
        let ay: i32 = game.get(2).unwrap().as_str().parse().unwrap();
        let bx: i32 = game.get(3).unwrap().as_str().parse().unwrap();
        let by: i32 = game.get(4).unwrap().as_str().parse().unwrap();
        let px: i32 = game.get(5).unwrap().as_str().parse().unwrap();
        let py: i32 = game.get(6).unwrap().as_str().parse().unwrap();
        let subtotal = match (0..=100)
            .cartesian_product(0..=100)
            .filter(|(a, b)| a * ax + b * bx == px)
            .filter(|(a, b)| a * ay + b * by == py)
            .map(|(a, b)| 3 * a + b)
            .min()
        {
            Some(tickets) => tickets,
            None => 0,
        };
        total += subtotal;
    }
    dbg!(total);
}
fn part2(input: String) {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let mut total = 0;
    for game in re.captures_iter(&input) {
        let ax: f64 = game.get(1).unwrap().as_str().parse().unwrap();
        let ay: f64 = game.get(2).unwrap().as_str().parse().unwrap();
        let bx: f64 = game.get(3).unwrap().as_str().parse().unwrap();
        let by: f64 = game.get(4).unwrap().as_str().parse().unwrap();
        let px: f64 = game.get(5).unwrap().as_str().parse::<f64>().unwrap() + 10000000000000.0;
        let py: f64 = game.get(6).unwrap().as_str().parse::<f64>().unwrap() + 10000000000000.0;

        // If py/px == by/bx, you know that you only need to press B to get to the prize
        // My method is to find the amount of As you need to press to get the remainding prize to
        // fit on that line (by/bx)
        // You I found that by finding "a" such that
        // rem_y = py - a * ay
        // rem_x = px - a * ax
        // and rem_y / rem_x = by / bx
        // px, py, ax, ay, bx, by we know, rem_y and rem_y we don't need the values of.
        // This leads to the followg equation:
        let a = (by * px - bx * py) / (by * ax - bx * ay);

        // We know how many a-presses we need, b-presses are then trivial:
        let b = (px - a * ax) / bx;

        // Finally we need to ignore the solutions that lead to negative presses or floating
        // presses (not possible with the machine):
        let subtotal = if a >= 0.0 && b >= 0.0 && a.floor() == a && b.floor() == b {
            (3.0 * a + b) as i64
        } else {
            0
        };

        total += subtotal;
    }
    dbg!(total);
}
