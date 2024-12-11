use std::{collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input:String){
    let mut total = 0;
    for (i,line) in input.lines().enumerate() {
        let chars:Vec<_> = line.split(" ").next().unwrap().chars().collect();
        let groups: Vec<usize> = line.split(" ").nth(1).unwrap().split(",").map(|n| n.parse().unwrap()).collect();
        let damaged_to_find = groups.iter().copied().sum::<usize>() - chars.iter().copied().filter(|c| *c == '#').count();
        let damaged_hyps: Vec<_> = chars.iter().copied()
        .enumerate()
        .filter(|(_,c)| *c == '?')
        .map(|(i,_)| i)
        .combinations(damaged_to_find)
        .collect();
        for hyp in damaged_hyps {
            let mut in_group = false;
            let mut found_groups = Vec::new();
            
            for (i, c) in chars.iter().enumerate() {
                if *c == '#'  || hyp.contains(&i) {
                    if in_group {
                        *found_groups.last_mut().unwrap() += 1;
                    } else {
                        in_group = true;
                        found_groups.push(1);
                    }
                } else {
                    in_group = false;
                }
            }
            if groups == found_groups {
                total += 1;
            }
        }
    }
    dbg!(total);
}
fn part2(input:String){
    let mut total: u64 = 0;
    for (i,line) in input.lines().enumerate() {
        let chars:Vec<_> = (line.split(" ").next().unwrap().to_owned()+"?").repeat(5).strip_suffix("?").unwrap().chars().collect();
        let groups: Vec<usize> = (line.split(" ").nth(1).unwrap().to_owned()+",").repeat(5).strip_suffix(",").unwrap().split(",").map(|n| n.parse().unwrap()).collect();
        let mut possibles:HashSet<String> = HashSet::from([String::from("")]);
        for char in chars {
            let mut next_possibles: HashSet<String> = HashSet::new();
            for possible in &possibles {
                let mut new_strings = Vec::new();
                if char == '.' || char == '?' {
                    new_strings.push(possible.clone() + ".");
                } 
                if char == '#' || char == '?' {
                    new_strings.push(possible.clone() + "#");
                }
                new_strings.iter().cloned().filter(|p| verify_possible(p, &groups)).for_each(|p| {next_possibles.insert(p);});
            }
            possibles = next_possibles;
        }
        let final_possibles: Vec<_> = possibles.iter().cloned().filter(|p| verify_complete(p, &groups)).collect();
        total += final_possibles.len() as u64;
        dbg!(i);
    }
    dbg!(total);
}
fn verify_complete(start:&String, groups:&Vec<usize>) -> bool {
    let mut in_group = false;
    let mut found_groups = Vec::new();
    for c in start.chars() {
        if c == '#' {
            if in_group {
                *found_groups.last_mut().unwrap() += 1;
            } else {
                in_group = true;
                found_groups.push(1);
            }
        } else {
            in_group = false;
        }
    }
    found_groups == *groups
}
fn verify_possible(start:&String, groups:&Vec<usize>) -> bool {
    if start.is_empty() {return true;}
    let mut in_group = false;
    let mut found_groups = Vec::new();
    for c in start.chars() {
        if c == '#' {
            if in_group {
                *found_groups.last_mut().unwrap() += 1;
            } else {
                in_group = true;
                found_groups.push(1);
            }
        } else {
            in_group = false;
        }
    }
    if found_groups.is_empty() {return true;}
    for i in 0..found_groups.len()-1 {
        if !found_groups.get(i).unwrap() == *groups.get(i).unwrap() {return false;}
    }
    in_group && found_groups.last() <= groups.get(found_groups.len()-1) || found_groups.last() == groups.get(found_groups.len()-1)
}

