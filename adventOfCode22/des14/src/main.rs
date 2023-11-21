use std::fs;
use std::collections::HashSet;
use std::cmp::max;
use std::cmp::min;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut occ_locations: HashSet<(i32,i32)> = HashSet::new();

    //Fill in rock
    for line in input.lines() {
        let edges: Vec<(i32,i32)> = line.split("->")
            .map(|coord_s| {
                let coord_vec: Vec<i32> = coord_s.split(",")
                    .map(|coord| coord.trim().parse::<i32>().unwrap())
                    .collect();
                (coord_vec[0], coord_vec[1])
            }).collect();
        for i in 0..edges.len()-1 {
            if edges[i].0 == edges[i+1].0 {
                for y in min(edges[i].1, edges[i+1].1)..=max(edges[i].1,edges[i+1].1){
                    occ_locations.insert((edges[i].0, y));        
                }
            }
            else {
                for x in min(edges[i].0, edges[i+1].0)..=max(edges[i].0,edges[i+1].0){
                    occ_locations.insert((x, edges[i].1));        
                }
            }
        }
    }

    //Make sand fall
    let mut sand_to_rest = 0;
    'sim: loop {
        let mut sand_pos = (500,0);
        'grain: loop {
            if sand_pos.1 >= 163 {
                occ_locations.insert(sand_pos);
                break 'grain;
            }

            for delt in vec![0, -1, 1] {
                let new_pos = (sand_pos.0+delt, sand_pos.1+1);
                if !occ_locations.contains(&new_pos) {
                    sand_pos = new_pos;
                    continue 'grain;
                }
            }
            occ_locations.insert(sand_pos);
            sand_to_rest += 1;
            if sand_pos.1 == 0 {
                break 'sim;
            }
            break 'grain;
        }
    }
    dbg!(sand_to_rest);
}
