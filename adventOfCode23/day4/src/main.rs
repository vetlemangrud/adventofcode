use std::{collections::HashSet, fs, iter::repeat};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input:String) {
    let slice_len = input.lines().next().unwrap().find(":").unwrap() + 1;
    let mut points = 0;
    for line in input.lines() {
        let winning: HashSet<i32> = line[slice_len..].split("|").next().unwrap().replace("  ", " 0").trim().split(" ").map(|s| s.parse().unwrap()).collect();
        let numbers: HashSet<i32> = line[slice_len..].split("|").nth(1).unwrap().replace("  ", " 0").trim().split(" ").map(|s| s.parse().unwrap()).collect();
        match (winning.intersection(&numbers).count() as u32).checked_sub(1) {
            Some(p) =>    points += 2_i32.pow(p),
            None => ()
        }
    }
    dbg!(points);
}
fn part2(input:String) {
    let slice_len = input.lines().next().unwrap().find(":").unwrap() + 1;

    let mut amount:Vec<i32> = repeat(1).take(input.lines().count()).collect();
    let mut total = 0;
    for (i,line) in input.lines().enumerate() {
        let winning: HashSet<i32> = line[slice_len..].split("|").next().unwrap().replace("  ", " 0").trim().split(" ").map(|s| s.parse().unwrap()).collect();
        let numbers: HashSet<i32> = line[slice_len..].split("|").nth(1).unwrap().replace("  ", " 0").trim().split(" ").map(|s| s.parse().unwrap()).collect();
        let correct = winning.intersection(&numbers).count();
        let increase_by = *amount.get(i).unwrap();
        total += increase_by;
        for j in i+1..i+1+correct {
            if amount.len() <= j {break;}
            let old_amount = amount[j];
            amount[j] = old_amount + increase_by;
        }
    }
    dbg!(total);
}