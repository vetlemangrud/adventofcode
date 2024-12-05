use core::num;
use std::{collections::{HashMap, HashSet}, fs};

use regex::Regex;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn part1(input:String) {

    // If you try to run this, it probalby won't work because I padded the input with dots (Yes it's allowed to change the input directly, what you gonna do about it?)
    let width = input.lines().next().unwrap().len();
    let test_regex = Regex::new(r"(?s)(?<=\.{5}.{8}\.)\d{3}(?=\..{8}\.{5})|(?s)(?<=\.{4}.{9}\.)\d{2}(?=\..{9}\.{4})|(?s)(?<=\.{3}.{10}\.)\d{1}(?=\..{10}\.{3})").unwrap();
    let actual_regex = Regex::new(r"(?s)(?<=\.{5}.{138}\.)\d{3}(?=\..{138}\.{5})|(?s)(?<=\.{4}.{139}\.)\d{2}(?=\..{139}\.{4})|(?s)(?<=\.{3}.{140}\.)\d{1}(?=\..{140}\.{3})").unwrap();

    let re = match width {
        12 => test_regex,
        _ => actual_regex
    };
    dbg!(re.find(&input));
    // Here i found out that rust does not support lookahead/lookbehind in regex. I did part 1 in Python instead
    
}

fn part2(input: String){
    let mut coord2stri:HashMap<(usize,usize),usize> = HashMap::new();
    let mut strings: Vec<String> = Vec::new();
    let mut gears: HashSet<(usize,usize)> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                match coord2stri.get_mut(&(x - 1, y)) {
                    None => {
                        coord2stri.insert((x, y), strings.len());
                        strings.push(String::from(char) as String);
                    },
                    Some(stri) => {
                        strings.get_mut(*stri).unwrap().push(char);
                        let i = *stri;
                        coord2stri.insert((x, y),i);
                    }
                }
            } else if char == '*' {
                gears.insert((x, y));
            }
        }
    }

    let nums:Vec<i32> = strings.iter().map(|s|s.parse().unwrap()).collect();

    let offsets:Vec<(i32,i32)> = vec![(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),(1,-1)];
    let mut total = 0;
    for gear in gears {
        let mut surrounding:HashSet<usize> = HashSet::new();
        for offset in &offsets {
            coord2stri.get(&((gear.0 as i32 + offset.0) as usize,(gear.1 as i32 + offset.1) as usize)).and_then(|i| Some(surrounding.insert(*i)));
        }
        if surrounding.len() == 2 {
            total += surrounding.iter().fold(1, |acc, i| acc * nums.get(*i as usize).unwrap());
        }
    }
    dbg!(total);
}