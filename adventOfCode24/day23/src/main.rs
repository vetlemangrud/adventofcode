use std::{
    collections::{HashMap, HashSet},
    fs,
};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut connected = HashMap::new();
    let re = Regex::new(r"(..)-(..)").unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let pc1 = captures.get(1).unwrap().as_str();
        let pc2 = captures.get(2).unwrap().as_str();
        add_connection(pc1, pc2, &mut connected);
        add_connection(pc2, pc1, &mut connected);
    }
    let mut groups = HashSet::new();
    for pc1 in connected.keys().filter(|pc| pc.starts_with("t")) {
        for pc2 in connected.get(pc1).unwrap() {
            for pc3 in connected.get(pc2).unwrap() {
                for pc4 in connected.get(pc3).unwrap() {
                    if pc1 == pc4 {
                        let mut pcs = Vec::from([pc1, pc2, pc3]);
                        pcs.sort();
                        groups.insert((pcs[0], pcs[1], pcs[2]));
                    }
                }
            }
        }
    }
    dbg!(groups.len());
}
fn part2(input: String) {
    let mut connected = HashMap::new();
    let re = Regex::new(r"(..)-(..)").unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let pc1 = captures.get(1).unwrap().as_str();
        let pc2 = captures.get(2).unwrap().as_str();
        add_connection(pc1, pc2, &mut connected);
        add_connection(pc2, pc1, &mut connected);
    }
    let mut lans: Vec<Vec<&str>> = Vec::new();
    for pc in connected.keys() {
        let others = connected.get(pc).unwrap();
        let mut new_lans = Vec::new();
        'lan: for lan in lans.iter() {
            for pc2 in lan {
                if !others.contains(pc2) {
                    continue 'lan;
                }
            }
            let mut new_lan = lan.clone();
            new_lan.push(pc);
            new_lans.push(new_lan);
        }
        new_lans.push(Vec::from([*pc]));
        lans.extend(new_lans);
    }
    let mut best = lans.iter().max_by_key(|l| l.len()).unwrap().clone();
    best.sort();
    dbg!(best.join(","));
}
fn add_connection<'a>(pc1: &'a str, pc2: &'a str, map: &mut HashMap<&'a str, Vec<&'a str>>) {
    if !map.contains_key(pc1) {
        map.insert(pc1, Vec::new());
    }
    map.get_mut(pc1).unwrap().push(pc2);
}
