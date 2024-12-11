use std::{collections::{HashMap, HashSet}, fs, hash::Hash, str::Chars};

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
    // Struggled a lot with part 2. Watched HyperNeutrinos solve of this part, and my solution is therefore very similar
    let mut total = 0;
    for line in input.lines() {
        let chars:Vec<_> = (line.split(" ").next().unwrap().to_owned()+"?").repeat(5).strip_suffix("?").unwrap().chars().collect();
        let groups: Vec<usize> = (line.split(" ").nth(1).unwrap().to_owned()+",").repeat(5).strip_suffix(",").unwrap().split(",").map(|n| n.parse().unwrap()).collect();
        let mut map = HashMap::new();
        let subtotal = get_possible_count(&chars, 0, &groups,0, &mut map);
        total += subtotal;
    }
    dbg!(total);
}
fn get_possible_count(chars: &Vec<char>, chars_i:usize, groups: &Vec<usize>, groups_i:usize, map: &mut HashMap<(usize,usize),i64>) -> i64 {
    if groups.len() <= groups_i{
        if chars.iter().copied().skip(chars_i).all(|c| c != '#') {return 1;} else {return 0;}
    }
    else if chars.len() <= chars_i {return 0;}
    match map.get(&(chars_i, groups_i)) {
       Some(v) => return *v,
       None => (), 
    }

    let mut total = 0;
    if chars[chars_i] == '.' || chars[chars_i] == '?' {
        total += get_possible_count(chars, chars_i+1, groups, groups_i, map);
    }
    if chars[chars_i] == '#' || chars[chars_i] == '?' {
        let next_chars = chars.iter().skip(chars_i).copied();
        let enough_left = chars.len() - chars_i >= groups[groups_i];
        let next_n_broken = next_chars.clone().take(groups[groups_i]).all(|c| c == '#' || c == '?');
        let fixed_after = next_chars.skip(groups[groups_i]).take(1).all(|c| c== '.' || c== '?') ;
        if enough_left && next_n_broken && fixed_after{
            total += get_possible_count(chars, chars_i + groups[groups_i]+1, groups, groups_i + 1, map);
        }
    }
    map.insert((chars_i,groups_i), total);
    total
}