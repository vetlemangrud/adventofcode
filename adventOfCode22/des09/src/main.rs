use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut directions: HashMap<&str, (i32, i32)> = HashMap::new();
    directions.insert("R", (1,0));
    directions.insert("L", (-1,0));
    directions.insert("U", (0,1));
    directions.insert("D", (0,-1));
    const ROPE_LENGTH: u32 = 10;
    let mut positions: Vec<(i32,i32)> = Vec::new();
    for _ in 0..ROPE_LENGTH {positions.push((0,0))};
    let mut thistory: HashSet<(i32, i32)> = HashSet::new();
    thistory.insert((0,0));
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let direction = directions.get(parts[0]).unwrap();
        for _ in 0..parts[1].parse::<u32>().unwrap() {
            positions[0].0 += direction.0;
            positions[0].1 += direction.1;
            for i in 1..positions.len() {
                if (positions[i-1].0 - positions[i].0).abs() <= 1 && (positions[i-1].1 - positions[i].1).abs() <=1 {
                    continue;
                }
                let mut tmove = (0,0);
                if positions[i-1].0 < positions[i].0 {
                    tmove.0 = -1;
                }
                if positions[i-1].0 > positions[i].0 {
                    tmove.0 = 1;
                }
                if positions[i-1].1 < positions[i].1 {
                    tmove.1 = -1;
                }
                if positions[i-1].1 > positions[i].1 {
                    tmove.1 = 1;
                }
                positions[i].0 += tmove.0;
                positions[i].1 += tmove.1;
            }
            thistory.insert(positions[positions.len()-1]);
        }
    }
    dbg!(thistory.len());

}
