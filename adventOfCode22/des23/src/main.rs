use std::{collections::HashMap, fs};
use std::collections::{vec_deque, HashSet, VecDeque};
use itertools::Itertools;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn part1(input: String) {
    let mut positions : HashSet<(i32,i32)> = HashSet::new();
    
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                positions.insert((x as i32, y as i32));
            }
        }
    }
    let mut proposals : HashMap<(i32,i32),Option<(i32,i32)>> = HashMap::new();
    let mut moves = VecDeque::from([
        vec![(0,-1),(-1,-1),(1,-1)], // N
        vec![(0,1),(-1,1),(1,1)], // S
        vec![(-1,0),(-1,-1),(-1,1)], // W
        vec![(1,0),(1,-1),(1,1)], // E
    ]);
    
    for _ in 0..10 {
        
        proposals.clear();

        // Get proposals
        for p in &positions {
            // Lonely boys don't propose
            if ![-1,-1,0,1,1].iter().permutations(2).any(|o| {
                positions.contains(&(p.0 + **o.get(0).unwrap(),p.1+**o.get(1).unwrap()))
            }) {continue;}

            'mov: for m in &moves {
                for o in m {
                    if positions.contains(&(p.0+o.0,p.1 +o.1)) {
                        continue 'mov;
                    }
                }
                let proposed_move =(p.0 + m.first().unwrap().0, p.1 + m.first().unwrap().1);
                match proposals.get(&proposed_move) {
                    None => {proposals.insert( proposed_move,Some(*p));},
                    Some(_) => {proposals.insert(proposed_move, None);}
                };
                break;
            }
        }

        // Move if possible
        for (new_pos, old_pos) in &proposals {
            match old_pos {
                Some(pos) => {
                    positions.remove(pos);
                    positions.insert(*new_pos);
                }
                None => ()
            }
        }


        // Shift moves
        let m = moves.pop_front().unwrap();
        moves.push_back(m);
    }

    // Calc empty spaces
    let minX = positions.iter().map(|p|p.0).min().unwrap();
    let maxX = positions.iter().map(|p|p.0).max().unwrap();
    let minY = positions.iter().map(|p|p.1).min().unwrap();
    let maxY = positions.iter().map(|p|p.1).max().unwrap();
    let empty = (maxX-minX + 1) * (maxY - minY + 1) - positions.iter().count() as i32;
    dbg!(empty);


}

fn part2(input: String) {
    let mut positions : HashSet<(i32,i32)> = HashSet::new();
    
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                positions.insert((x as i32, y as i32));
            }
        }
    }
    let mut proposals : HashMap<(i32,i32),Option<(i32,i32)>> = HashMap::new();
    let mut moves = VecDeque::from([
        vec![(0,-1),(-1,-1),(1,-1)], // N
        vec![(0,1),(-1,1),(1,1)], // S
        vec![(-1,0),(-1,-1),(-1,1)], // W
        vec![(1,0),(1,-1),(1,1)], // E
    ]);
    
    let mut round = 0;
    loop {
        round += 1; 
        proposals.clear();

        // Get proposals
        for p in &positions {
            // Lonely boys don't propose
            if ![-1,-1,0,1,1].iter().permutations(2).any(|o| {
                positions.contains(&(p.0 + **o.get(0).unwrap(),p.1+**o.get(1).unwrap()))
            }) {continue;}

            'mov: for m in &moves {
                for o in m {
                    if positions.contains(&(p.0+o.0,p.1 +o.1)) {
                        continue 'mov;
                    }
                }
                let proposed_move =(p.0 + m.first().unwrap().0, p.1 + m.first().unwrap().1);
                match proposals.get(&proposed_move) {
                    None => {proposals.insert( proposed_move,Some(*p));},
                    Some(_) => {proposals.insert(proposed_move, None);}
                };
                break;
            }
        }

        // Check for end condition
        if proposals.values().all(|p| p.is_none()) {
            break;
        }

        // Move if possible
        for (new_pos, old_pos) in &proposals {
            match old_pos {
                Some(pos) => {
                    positions.remove(pos);
                    positions.insert(*new_pos);
                }
                None => ()
            }
        }


        // Shift moves
        let m = moves.pop_front().unwrap();
        moves.push_back(m);
    }

   
    dbg!(round);


}

fn draw_pos(positions: &HashSet<(i32,i32)>) {
    let mut out = String::new();
    for y in 0..12 {
        for x in 0..12 {
            if positions.contains(&(x,y)) {
                out += "#";
            } else {
                out += ".";
            }
        }
        out += "\n";
    }
    fs::write("test_out", out);
}