use std::{collections::{HashMap, HashSet}, fs};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input:String) {
    // Parse input
    let mut blizzards = 0;
    let mut positions: HashMap<i32,(i32,i32)> = HashMap::new();
    let mut directions: HashMap<i32,(i32,i32)> = HashMap::new();
    let height = (input.lines().count() -2) as i32;
    let width = (input.lines().next().unwrap().len() -2) as i32;
    let mut possible_positions = HashSet::from([(0,-1)]);
    let moves = vec![(0,0),(0,1),(1,0),(0,-1),(-1,0)];

    for (y, line) in input.lines().skip(1).enumerate() {
        for (x,char) in line.chars().skip(1).enumerate(){
            match char {
                '>' => {directions.insert(blizzards, (1,0));},
                'v' => {directions.insert(blizzards, (0,1));},
                '<' => {directions.insert(blizzards, (-1,0));},
                '^' => {directions.insert(blizzards, (0,-1));},
                _ => continue,
            }
            positions.insert(blizzards, (x as i32, y as i32));
            blizzards += 1;
        }
    }

    let mut minutes = 0;

    loop {
        // Calculate next board state
        for id in 0..blizzards {
            let p = positions.get(&id).unwrap();
            let dir = directions.get(&id).unwrap();
            let new_pos = (((p.0 + dir.0) + width)%width, ((p.1 + dir.1) + height)%height);
            positions.insert(id, new_pos);
        }

        // Calculate possible new positions
        let new_possible = possible_positions.iter().flat_map(|old_pos| {
            moves.iter()
            .map(|m| (old_pos.0 + m.0, old_pos.1 + m.1)) // Get positions after each move
            .filter(|p| 0 <= p.0 && p.0 < width && 0 <= p.1 && (p.1 < height || p.0 == width -1))// Move is within bounds
            .filter(|p| ! positions.values().any(|b_p| b_p == p)) // Move does not crash with blizzard
        });

        possible_positions = new_possible.collect();
        minutes += 1;

        // Check win condition
        if possible_positions.iter().any(|p| p.1 == height) {
           break; 
        }
    }
    dbg!(minutes);
}

fn part2(input:String) {
    // Parse input
    let mut blizzards = 0;
    let mut positions: HashMap<i32,(i32,i32)> = HashMap::new();
    let mut directions: HashMap<i32,(i32,i32)> = HashMap::new();
    let height = (input.lines().count() -2) as i32;
    let width = (input.lines().next().unwrap().len() -2) as i32;
    let mut possible_positions = HashSet::from([(0,-1)]);
    let moves = vec![(0,0),(0,1),(1,0),(0,-1),(-1,0)];
    let mut goals = vec![(width-1, height),(0,-1),(width-1,height)];

    for (y, line) in input.lines().skip(1).enumerate() {
        for (x,char) in line.chars().skip(1).enumerate(){
            match char {
                '>' => {directions.insert(blizzards, (1,0));},
                'v' => {directions.insert(blizzards, (0,1));},
                '<' => {directions.insert(blizzards, (-1,0));},
                '^' => {directions.insert(blizzards, (0,-1));},
                _ => continue,
            }
            positions.insert(blizzards, (x as i32, y as i32));
            blizzards += 1;
        }
    }

    let mut minutes = 0;

    loop {
        // Calculate next board state
        for id in 0..blizzards {
            let p = positions.get(&id).unwrap();
            let dir = directions.get(&id).unwrap();
            let new_pos = (((p.0 + dir.0) + width)%width, ((p.1 + dir.1) + height)%height);
            positions.insert(id, new_pos);
        }

        // Calculate possible new positions
        let new_possible = possible_positions.iter().flat_map(|old_pos| {
            moves.iter()
            .map(|m| (old_pos.0 + m.0, old_pos.1 + m.1)) // Get positions after each move
            .filter(|p| (0 <= p.0 && p.0 < width && 0 <= p.1 && p.1 < height) || p == &(0,-1) || p == &(width-1,height))// Move is within bounds
            .filter(|p| ! positions.values().any(|b_p| b_p == p)) // Move does not crash with blizzard
        });


        possible_positions = new_possible.collect();
        minutes += 1;

        // Check goal condition
        if possible_positions.iter().any(|p| p == goals.last().unwrap()) {
            possible_positions.clear();
            possible_positions.insert(goals.pop().unwrap());
                            
            if goals.len() == 0 {
                break; 
            }
        }
        if possible_positions.iter().count() == 0 {
            print!("no possible positions");
            break;
        }

        if minutes % 100 == 0 {
            dbg!(&minutes);
        }
    }
    dbg!(minutes);
}
