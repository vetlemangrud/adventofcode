use std::fs;
use std::collections::HashMap;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    let directions = vec![(1,0),(0,1),(-1,0),(0,-1)];
    let mut dir = 0;
    let mut map: HashMap<(u32,u32), char> = HashMap::new();
    for (y, line) in input.split("\n\n").take(1).next().unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == ' ' {continue;}
           map.insert((x as u32,y as u32),c); 
        }
    }
    let pos =( map.keys().filter(|p| p.1 == 0).map(|p| p.0).min().unwrap(), 0);
   
    // Map to hold relations between squares (sqare coordinate, direction) -> (square, relative
    // rotation)
    let mut relations: HashMap<((u32,u32), (i32,i32)), (u32, u32)> = HashMap::new();

    for y in 0..4 {
        for x in 0..3 {
            if map.contains_key(&(x*50, y*50)) {
               println!("{} {}", x, y); 
            }
        }
    }
}
