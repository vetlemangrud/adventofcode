use std::{cmp::Ordering, collections::{HashMap, HashSet}, fs};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn part1(input:String) {
    let score:i32 = input.lines()
    .map(|l| l.split(" ").collect_tuple::<(&str, &str)>().unwrap())
    .map(|l| (l.0, l.1.parse::<i32>().unwrap()))
    .sorted_by(|a,b| compare_hands(a.0, b.0))
    .enumerate()
    .map(|(i, l)| (i as i32+1)* l.1)
    .sum();
dbg!(score);
}

fn part2(input:String) {
    let score:i32 = input.lines()
    .map(|l| l.split(" ").collect_tuple::<(&str, &str)>().unwrap())
    .map(|l| (l.0, l.1.parse::<i32>().unwrap()))
    .sorted_by(|a,b| compare_hands_2(a.0, b.0))
    .enumerate()
    .map(|(i, l)| (i as i32+1)* l.1)
    .sum();
dbg!(score);
// 244965885 is too high
// 243502605 is too high
}

fn compare_hands(a:&str, b:&str) -> Ordering {
    let strength_map = HashMap::from([('2',2),('3',3),('4',4),('5',5),('6',6),('7',7),('8',8),('9',9),('T', 10),('J',11),('Q',12),('K',13),('A',14)]);

    let types : Vec<_> = [a,b].iter().map(|hand| {
        let labels: HashSet<_> = hand.chars().collect();
        let occurences: Vec<_> = labels.iter().map(|l| count_occurences(hand, l)).collect();
        // Five of a kind
        if labels.len() == 1 {6}
        // Four of a kind
        else if occurences.iter().any(|o| *o == 4) {5}
        // Full house
        else if occurences.iter().tuple_combinations().any(|(a,b)| *a + *b == 5) {4}
        // Three of a kind
        else if occurences.iter().any(|o| *o == 3) {3}
        // Two pair
        else if occurences.iter().tuple_combinations().any(|(a,b)| *a+ *b  == 4) {2}
        // One pair
        else if occurences.iter().any(|o| *o == 2) {1}
        // High card
        else {0}
    }).collect();
    if types[0] > types[1] {return Ordering::Greater}
    if types[0] < types[1] {return Ordering::Less}
    for (ac,bc) in a.chars().map(|c|strength_map.get(&c).unwrap()).zip(b.chars().map(|c| strength_map.get(&c).unwrap())) {
        if ac > bc {return Ordering::Greater}
        if ac < bc {return Ordering::Less}
    }
    Ordering::Equal
}


fn compare_hands_2(a:&str, b:&str) -> Ordering {
    let strength_map = HashMap::from([('2',2),('3',3),('4',4),('5',5),('6',6),('7',7),('8',8),('9',9),('T', 10),('J',1),('Q',12),('K',13),('A',14)]);

    let types : Vec<_> = [a,b].iter().map(|hand| {
        let labels: HashSet<_> = hand.chars().collect();
        let occurences: Vec<_> = labels.iter().filter(|c| **c != 'J').map(|l| count_occurences(hand, l)).collect();
        let js = count_occurences(hand, &'J');
        // Five of a kind
        if occurences.len() <= 1 {6}
        // Four of a kind
        else if occurences.iter().any(|o| *o + js == 4) {5}
        // Full house
        else if occurences.iter().tuple_combinations().any(|(a,b)| *a + *b + js == 5) {4}
        // Three of a kind
        else if occurences.iter().any(|o| *o + js == 3) {3}
        // Two pair
        else if occurences.iter().tuple_combinations().any(|(a,b)| *a+ *b + js  == 4) {2}
        // One pair
        else if occurences.iter().any(|o| *o + js == 2) {1}
        // High card
        else {0}
    }).collect();
    if types[0] > types[1] {return Ordering::Greater}
    if types[0] < types[1] {return Ordering::Less}
    for (ac,bc) in a.chars().map(|c|strength_map.get(&c).unwrap()).zip(b.chars().map(|c| strength_map.get(&c).unwrap())) {
        if ac > bc {return Ordering::Greater}
        if ac < bc {return Ordering::Less}
    }
    Ordering::Equal
}

fn count_occurences(string: &str, char: &char)->i32 {
    string.chars().filter(|c| c == char).count() as i32
}
