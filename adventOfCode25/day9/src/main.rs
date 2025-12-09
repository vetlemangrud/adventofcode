use std::fs::{self};

use geo::Area;
use geo::Contains;
use geo::LineString;
use geo::Polygon;
use geo::Rect;
use geo::coord;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut tiles: Vec<(i64, i64)> = Vec::new();
    for line in input.lines() {
        let mut coords = line.split(",");
        tiles.push((
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
        ));
    }
    let combinations = tiles.iter().tuple_combinations();
    let max = combinations
        .map(|((x1, y1), (x2, y2))| ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1))
        .max();
    dbg!(max);
}

// // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
// fn intersects(
//     ((y1, x1), (x2, y2)): ((i64, i64), (i64, i64)),
//     ((y3, x3), (x4, y4)): ((i64, i64), (i64, i64)),
// ) -> Option<(i64, i64)> {
//     let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4))
//         / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));
//     let u = ((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3))
//         / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));
//     if (0 <= t && t <= 1 && 0 <= u && u <= 1) {
//         return Some((x1 + t, y1 + t * (y2 - y1)));
//     }
//     None
// }
fn cursom_area(rect: &Rect) -> i64 {
    (rect.width() as i64 + 1) * (rect.height() as i64 + 1)
}
fn part2(input: String) {
    let mut tiles: Vec<(f64, f64)> = Vec::new();
    for line in input.lines() {
        let mut coords = line.split(",");
        tiles.push((
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
        ));
    }
    let valid_floor = Polygon::new(LineString::from(tiles.clone()), vec![]);
    let mut rectanges = tiles
        .iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| Rect::new(coord! {x:*x1,y:*y1}, coord! {x:*x2,y:*y2}))
        .filter(|rect| valid_floor.contains(rect))
        .sorted_by_key(|rect| cursom_area(rect))
        .rev();
    let best = rectanges.next();
    println!("{}", cursom_area(&best.unwrap()));
}
