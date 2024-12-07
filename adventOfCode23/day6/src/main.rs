use std::{fs, iter::{self, zip}};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input:String) {
    let times = input.lines().next().unwrap().split(" ").flat_map(|s| s.parse().map_or(Vec::new(), |n:i32| Vec::from([n])));
    let dists = input.lines().nth(1).unwrap().split(" ").flat_map(|s| s.parse().map_or(Vec::new(), |n:i32| Vec::from([n])));
    let races = zip(times, dists);
    let total:i32 = races.map(|(time, record_dist)|
        (0..time)
        .map(|held_time| 
            held_time * (time-held_time)
        )
        .filter(|dist| 
            *dist > record_dist
        )
        .count() as i32
    )
    .product();
dbg!(total);
}

fn part2(input:String) {
    let time:i64 = input.lines().next().unwrap().split(":").nth(1).unwrap().replace(" ", "").parse().unwrap();
    let record_dist:i64 = input.lines().nth(1).unwrap().split(":").nth(1).unwrap().replace(" ", "").parse().unwrap();
    dbg!((time,record_dist));
    let total = (0..time)
        .map(|held_time| 
            held_time * (time-held_time)
        )
        .filter(|dist| 
            *dist > record_dist
        )
        .count() as i64;
    dbg!(total);
}