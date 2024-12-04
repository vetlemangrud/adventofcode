use std::fs;
use std::collections::HashMap;
use regex::Regex;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    
    let directions = vec![(1,0),(0,1),(-1,0),(0,-1)];//Right, Down, Left, Up
    let mut dir = 0;
    let mut map: HashMap<(i32,i32), char> = HashMap::new();
    let mut visited: HashMap<(i32,i32),i32> = HashMap::new();
    for (y, line) in input.split("\n\n").take(1).next().unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == ' ' {continue;}
           map.insert((x as i32 + 1,y as i32 + 1),c); 
        }
    }
    let mut pos =( map.keys().filter(|p| p.1 == 1).map(|p| p.0).min().unwrap() as i32, 1);

    // This sucks, man!
    let portals = vec![
        // (infrom,indelta,outfrom,outdelta,indir,outdir)
        ((151,1),(0,1),(101,150),(0,-1),0,2),
        ((101,51),(1,0),(101,51),(0,1),1,2),
        ((51,151),(1,0),(51,151),(0,1),1,2),
        ((1,201),(1,0),(101,0),(1,0),1,1),
        ((0,151),(0,1),(51,0),(1,0),2,1),
        ((0,101),(0,1),(50,50),(0,-1),2,0),
        ((1,100),(1,0),(50,51),(0,1),3,0),
        //Mirrored
        ((101,150),(0,-1),(151,1),(0,1),0,2),
        ((101,51),(0,1),(101,51),(1,0),0,3),
        ((51,151),(0,1),(51,151),(1,0),0,3),
        ((101,0),(1,0),(1,201),(1,0),3,3),
        ((51,0),(1,0),(0,151),(0,1),3,0),
        ((50,50),(0,-1),(0,101),(0,1),2,0),
        ((50,51),(0,1),(1,100),(1,0),2,1),

    ];

    let re = Regex::new(r"(\d+)(R|L)").unwrap();
    for (move_length, move_turn) in re.captures_iter(&(input+"R00L")).map(|m| (m.get(1).unwrap().as_str().parse::<i32>().unwrap(), m.get(2).unwrap().as_str())) {
        //Move
        // Try moving
        for _ in 0..move_length {
            // Get new position and direction when accounting for portals (edges of cube)
            let mut new_pos = get_new_pos(&pos, &dir, &directions);
            let mut new_dir = dir;
            if !map.contains_key(&new_pos) {
                // Portal time
                for p in &portals {
                    // Check if new_pos and new_dir follow conditions for entering portal
                    if new_dir != p.4 {continue;}
                    for offset in 0..50 {
                       if new_pos == (p.0.0 + p.1.0 * offset, p.0.1 + p.1.1 * offset) {
                            new_dir = p.5;
                            new_pos = get_new_pos(&(p.2.0 + p.3.0 * offset, p.2.1 + p.3.1 * offset), &new_dir, &directions);
                        break;
                       }
                    }
                }
            }
            // Check if able to move
            if map.get(&new_pos).unwrap() == &'.' {
               dir = new_dir; 
                visited.insert(pos, dir);
               pos = new_pos;
            }
        }
        //Turn
        match move_turn {
            "R" => dir = (dir + 1)%4,
            "L" => dir = (dir  +3)%4,
            _ => ()
        }
    }

    let mut mapdrawing = String::new();
    for y in 1..=200 {
        for x in 1..=150 {
            if visited.contains_key(&(x,y)) {
                mapdrawing += match visited.get(&(x,y)).unwrap() {
                    0 => ">",
                    1 => "V",
                    2 => "<",
                    3 => "^",
                    _ => "?"
                }
            } else {
                mapdrawing += &map.get(&(x,y)).unwrap_or(&' ').to_string();
            }
        }
        mapdrawing += "\n";
    } 
    fs::write("map.txt", mapdrawing);

    dbg!(pos);
    dbg!(dir);
    dbg!(1000*pos.1 + 4*pos.0 + dir);

   
}

fn get_new_pos(old_pos: &(i32,i32), direction:&i32, directions:&Vec<(i32,i32)>) -> (i32,i32){
    let dir_vector = directions.get(*direction as usize).unwrap();
    (old_pos.0 + dir_vector.0, old_pos.1 + dir_vector.1)
}
