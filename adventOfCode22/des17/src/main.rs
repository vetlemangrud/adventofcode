use std::fs;
use std::collections::HashSet;

fn height(blocks: &HashSet<(i32,i32)>) -> i32 {
    *blocks.iter().map(|(_,y)| y).max().unwrap_or(&-1) + 1
}

fn main() {
    let pieces_string = fs::read_to_string("pieces").unwrap();
    let piece_strings: Vec<&str> = pieces_string.split("\n\n").collect();
    let pieces: Vec<HashSet<(i32,i32)>> = piece_strings.iter()
        .map(|s|{
            let mut set: HashSet<(i32,i32)> = HashSet::new();
            for (y, l) in s.lines().enumerate() {
                for (x, c) in l.chars().enumerate() {
                    if c == '#' {set.insert((x as i32,s.lines().collect::<Vec<_>>().len() as i32 - (y as i32) - 1));}
                }
            }
            set
        }).collect();

    let moves_string = fs::read_to_string("input").unwrap();
    //let moves_string = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let mut moves: Vec<i32> = moves_string.chars()
        .filter(|m| *m == '<' || *m == '>')
        .map(|m| if m == '<' {-1} else {1}).collect();
    let mut occupied: HashSet<(i32,i32)> = HashSet::new();
    let mut piece_no = 0;
    let mut move_no = 0;
    let mut encountered = 0;
    loop {
        let mut curr_pos = (2, height(&occupied) + 3);
        loop {
            let m = moves[move_no%moves.len()];
            move_no += 1;
            if curr_pos.0 + m >= 0 && pieces[piece_no%5].iter().all(|p|
                curr_pos.0 + p.0 + m < 7 &&
                !occupied.contains(&(curr_pos.0+p.0+m,curr_pos.1+p.1))) {
                curr_pos.0 += m;
            }
            if curr_pos.1 > 0 && pieces[piece_no%5].iter().all(|p|
                !occupied.contains(&(curr_pos.0+p.0,curr_pos.1+p.1-1))) {
                curr_pos.1 -= 1;
            }
            else {
                for block in &pieces[piece_no%5] {
                    occupied.insert((curr_pos.0 + block.0, curr_pos.1 + block.1));
                }
                piece_no += 1;
                break;
            }
        }
        if piece_no == 1793 + 1127 {dbg!(height(&occupied));break;}
        if move_no % moves.len() == 343 && piece_no % 5 == 3 {
            dbg!(piece_no);
            dbg!(height(&occupied));
            if encountered == 9 {
                break;
            }
            encountered+=1;
        }
    }
}
