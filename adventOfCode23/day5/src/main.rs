use std::{fs, iter::zip};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let seeds: Vec<i64> = input.lines().next().unwrap()[7..].split(" ").map(|s| {s.parse().unwrap()}).collect();

    let mut values = seeds.clone();
    for category in input.split("\n\n").skip(1).map(|l| l.lines().skip(1)) {
        let maps:Vec<(i64, i64, i64)> = category.map(|c| {
            let nums: Vec<i64> = c.split(" ").map(|n| n.parse().unwrap()).collect();
            (nums[0],nums[1],nums[2])
        }).collect();

        let mut next_values: Vec<i64> = Vec::new();
        'values: for value in &values {
            
            for map in &maps {
                if *value >= map.1 && *value < map.1 + map.2 {
                    next_values.push(value + map.0 - map.1);
                    continue 'values;
                }
            }
            next_values.push(*value);
        }
        values = next_values;
    }
    dbg!(values.iter().min());
}

fn part2(input: String) {
    let seeds = input.lines().next().unwrap()[7..].split(" ").map(|s| {s.parse().unwrap()});
    let init_ranges:Vec<(i64,i64)> = seeds.clone().zip(seeds.skip(1)).step_by(2).collect();

    let mut ranges  = init_ranges.clone();
    for category in input.split("\n\n").skip(1).map(|l| l.lines().skip(1)) {
        let maps:Vec<(i64, i64, i64)> = category.map(|c| {
            let nums: Vec<i64> = c.split(" ").map(|n| n.parse().unwrap()).collect();
            (nums[0],nums[1],nums[2])
        }).collect();

        let mut next_ranges: Vec<(i64,i64)> = Vec::new();
        'values: for range in &ranges {
            let mut unmappeds = Vec::from([*range]);
            for map in &maps {
                let mut new_unmappeds:Vec<(i64,i64)> = Vec::new();
                for unmapped in &unmappeds {
                    let range_start = unmapped.0;
                    let range_end = unmapped.0 + unmapped.1;
                    let map_start = map.1;
                    let map_end = map.1 + map.2;
                    // Map totally covers range
                    if map_start <= range_start && map_end >= range_end {
                        next_ranges.push((range_start + map.0 - map.1, unmapped.1));
                        continue 'values;
                    }
                    // Map covers end of range
                    if map_start <= range_end && map_end >= range_end {
                        new_unmappeds.push((unmapped.0, map_start - range_start));
                        next_ranges.push((map.0, unmapped.1 - (map_start-range_start)));
                        break;
                    }
                    // Map covers start of range
                    if map_end >= range_start && map_start <= range_start {
                        new_unmappeds.push((map_end, range_end-map_end));
                        next_ranges.push((range_start + map.0 - map.1, unmapped.1 - (range_end-map_end)));
                        break;
                    }
                    // Map covers middle of range
                    if map_start > range_start && map_end < range_end {
                        next_ranges.push((map.0, map.2));
                        new_unmappeds.push((range_start, map_start-range_start));
                        new_unmappeds.push((map_end, range_end-map_end));
                        break;
                    }
                    new_unmappeds.push(*unmapped);
                }
                unmappeds = new_unmappeds;
            }
            next_ranges.append(&mut unmappeds);
        }
        ranges = next_ranges;
    }
    dbg!(ranges.iter().map(|r| r.0).min());
}
