use std::{collections::{HashMap, HashSet}, fs};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input:String) {
    let instr = input.split("\n\n").next().unwrap().chars().cycle();
    let node_re = Regex::new("(...) = .(...),.(...).").unwrap();
    let nodes: HashMap<_,_> = input
    .split("\n\n")
    .nth(1).unwrap()
    .lines()
    .map(|l| {
        let m = node_re.captures(l).unwrap();
        (m.get(1).unwrap().as_str(), (m.get(2).unwrap().as_str(),m.get(3).unwrap().as_str()))
    }).collect();
    
    let mut current = "AAA";
    let mut steps = 0;
    for i in instr {
        steps += 1;
        match i {
            'L' => current = nodes.get(&current).unwrap().0,
            _ => current = nodes.get(&current).unwrap().1,
        }
        if current == "ZZZ" {break;}
    }
    dbg!(steps);
}
fn part2(input:String) {
    let instr = input.split("\n\n").next().unwrap().chars().cycle();
    let node_re = Regex::new("(...) = .(...),.(...).").unwrap();
    let nodes: HashMap<_,_> = input
    .split("\n\n")
    .nth(1).unwrap()
    .lines()
    .map(|l| {
        let m = node_re.captures(l).unwrap();
        (m.get(1).unwrap().as_str(), (m.get(2).unwrap().as_str(),m.get(3).unwrap().as_str()))
    }).collect();
    
    let mut current: HashSet<_> = nodes.keys().map(|k| *k).filter(|k| k.ends_with("A")).collect();
    let mut goal_steps : HashSet<u64>= HashSet::new();
    let mut steps: u64 = 0;
    for i in instr {
        steps += 1;
        let new_current: HashSet<_> = current.iter().map(|n| {
            match i {
                'L' => nodes.get(*n).unwrap().0,
                _ => nodes.get(*n).unwrap().1,
            }
        }).collect();
        if new_current.iter().any(|s| s.ends_with("Z")) {goal_steps.insert(steps);}
        if new_current.iter().all(|s| s.ends_with("Z")) {break;}
        current = new_current.iter().filter(|s| !s.ends_with("Z")).map(|s| *s).collect();
    }
    dbg!(goal_steps);
}