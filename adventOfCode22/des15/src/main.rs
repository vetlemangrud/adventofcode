use std::fs;
use std::cmp::{max,min};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let sensors: Vec<(i32,i32,i32,i32)> = input.lines()
        .map(|line| {
            let mut coords = line.split(",")
             .map(|coord| coord.parse::<i32>().unwrap());
            (coords.next().unwrap(),
            coords.next().unwrap(),
            coords.next().unwrap(),
            coords.next().unwrap())
        }).collect();

    let y: i32 = 2000000;
    let mut ranges: Vec<(i32,i32)> = Vec::new();
    for sensor in sensors {
        let sensor_range=(sensor.0-sensor.2).abs()+(sensor.1-sensor.3).abs();
        let range_in_y = sensor_range - (sensor.1 - y).abs();
        if range_in_y > -1 {
            ranges.push((sensor.0-range_in_y, sensor.0+range_in_y));
        }
    }
    dbg!(&ranges);
    let mut total = 0;
    for range in &ranges {
        total += range.1 - range.0 + 1;
    }
    for i in 0..ranges.len() {
        for j in i+1..ranges.len() {
            total -= max(0, 1 + min(ranges[i].1,ranges[j].1)-max(ranges[i].0,ranges[j].0));
        }
    }
    dbg!(total);
    //Not 4353354
}
