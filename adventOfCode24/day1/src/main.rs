use std::{fs, iter::zip};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn part1(input: String) {
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    for line in input.lines() {
        let mut split = line.split("   ");
        list1.push(split.next().unwrap().parse().unwrap());
        list2.push(split.next().unwrap().parse().unwrap());
    }
    list1.sort();
    list2.sort();

    let mut total_distance = 0;
    for comb in zip(list1, list2) {
        total_distance += (comb.0 - comb.1).abs();
    }
    println!("{}", total_distance);
}

fn part2(input: String) {
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    for line in input.lines() {
        let mut split = line.split("   ");
        list1.push(split.next().unwrap().parse().unwrap());
        list2.push(split.next().unwrap().parse().unwrap());
    }

    let mut total = 0;
    for elem in list1 {
        total += elem * list2.iter().filter(|n| **n == elem).count() as i32;
    }
    println!("{}", total);
}
