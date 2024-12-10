use std::{collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    parts(input, 1_000_000);
}
fn parts(input:String, expand_size: i64) {
    let mut galaxies = HashSet::new();
    for (y, line) in input.lines().enumerate(){
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert((x as i64,y as i64));
            }
        }
    }
    let expanded_rows: HashSet<_> = (0..galaxies.iter().copied().map(|(_,y)| y).max().unwrap()).filter(|c| galaxies.iter().all(|(_,y)| y != c)).collect();
    let expanded_cols: HashSet<_> = (0..galaxies.iter().copied().map(|(x,_)| x).max().unwrap()).filter(|r| galaxies.iter().all(|(x,_)| x != r)).collect();
    let mut total_len = 0;
    for ((ax,ay),(bx,by)) in galaxies.iter().tuple_combinations::<(_,_)>() {
        let len_wo_expand = (bx-ax).abs() + (by-ay).abs();
        let expanded = expanded_cols.iter().cloned().filter(|c| c < ax.max(bx) && c > ax.min(bx)).count() + expanded_rows.iter().cloned().filter(|r| r < ay.max(by) && r > ay.min(by)).count();
        let l = len_wo_expand + (expand_size - 1) * expanded as i64;
        total_len += l;
    }
    dbg!(total_len);
}
