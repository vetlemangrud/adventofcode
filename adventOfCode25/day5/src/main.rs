use std::fs;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn is_included(id: i64, range: &(i64, i64)) -> bool {
    return range.0 <= id && range.1 >= id;
}

fn part1(input: String) {
    let mut ranges = Vec::<(i64, i64)>::new();
    let mut lines = input.lines().into_iter();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut ends = line.split("-");
        ranges.push((
            ends.next().unwrap().parse().unwrap(),
            ends.next().unwrap().parse().unwrap(),
        ));
    }
    let mut fresh_count = 0;
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let id: i64 = line.unwrap().parse().unwrap();
        if ranges.iter().any(|rng| is_included(id, rng)) {
            fresh_count += 1
        }
    }
    dbg!(fresh_count);
}

fn part2(input: String) {
    let mut ranges = Vec::<(i64, i64)>::new();
    let mut lines = input.lines().into_iter();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut ends = line.split("-");
        ranges.push((
            ends.next().unwrap().parse().unwrap(),
            ends.next().unwrap().parse().unwrap(),
        ));
    }
    let mut to_count = Vec::<(i64, i64)>::new();
    'outer: loop {
        if ranges.is_empty() {
            break;
        }
        let current = ranges.pop().unwrap();
        for prev in to_count.clone().iter() {
            if prev.0 > current.1 || prev.1 < current.0 {
                continue;
            }
            if prev.0 > current.0 {
                ranges.push((current.0, prev.0 - 1))
            }
            if prev.1 < current.1 {
                ranges.push((prev.1 + 1, current.1))
            }
            continue 'outer;
        }
        to_count.push(current);
    }
    let mut count = 0;
    for rng in to_count {
        count += rng.1 - rng.0 + 1;
    }
    dbg!(&count);
}
