use std::fs;
use std::collections::HashMap;
use std::cmp::max;
fn main() {
    const WIDTH: u32 = 99;
    const HEIGHT: u32 = 99;
    let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let input = fs::read_to_string("input").unwrap();

    let mut heights: HashMap<(u32, u32), u32> = HashMap::new();
    for (y, line) in input.lines().enumerate(){
        for (x, c) in line.chars().enumerate() {
            let height: u32 = c.to_digit(10).unwrap();
            heights.insert((x.try_into().unwrap(), y.try_into().unwrap()), height);
        }
    }

    
    let mut best = 1;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut scenic_score = 1;
            for (dir_x, dir_y) in &directions {
                let mut curr_pos = (x, y);
                let mut tree_count = 0;
                loop {
                    curr_pos.0 = (curr_pos.0 as i32 + dir_x) as u32;
                    curr_pos.1 = (curr_pos.1 as i32 + dir_y) as u32;
                    if curr_pos.0 < 0 || curr_pos.1 < 0 || curr_pos.0 >= WIDTH || curr_pos.1 >= HEIGHT {
                        break;
                    } 
                    tree_count += 1;
                    if heights.get(&(x, y)).unwrap() <= heights.get(&curr_pos).unwrap() {
                        break;
                    }
                }
                scenic_score *= tree_count;
            }
            if (scenic_score == 54) {
                dbg!((x,y));
            }
            best = max(best, scenic_score);
        }
    }
    dbg!(best);

    let num_visible = heights.keys()
        .filter(|tree|{
            let covered = heights.keys().any(|other| other.0 < tree.0 && other.1 == tree.1 && heights.get(other).unwrap() >= heights.get(tree).unwrap()) &&
                          heights.keys().any(|other| other.0 > tree.0 && other.1 == tree.1 && heights.get(other).unwrap() >= heights.get(tree).unwrap()) &&
                          heights.keys().any(|other| other.0 == tree.0 && other.1 < tree.1 && heights.get(other).unwrap() >= heights.get(tree).unwrap()) &&
                          heights.keys().any(|other| other.0 == tree.0 && other.1 > tree.1 && heights.get(other).unwrap() >= heights.get(tree).unwrap()); 
            !covered
        })
        .count();
    dbg!(num_visible); 
}
