use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut tiles = HashMap::new();
    let mut start = (0,0);
    for (y,line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {start = (x as i32,y as i32);}
            tiles.insert((x as i32,y as i32), c);
        }
    }
    let mut visited = HashSet::from([start]);
    let mut loop_border = HashSet::from([start]);
    let mut steps = 0;
    loop {
        let next_border: HashSet<_> = loop_border.iter()
        .flat_map(|(x,y)| get_connected(*x, *y, tiles.clone()))
        .filter(|p| !visited.contains(p))
        .collect();
        loop_border = next_border;
        loop_border.iter().for_each(|p| {visited.insert(*p);});
        steps += 1;
        if loop_border.len() == 1 {break;}

    }
    dbg!(steps);
}
fn part2(input: String) {
    let mut tiles = HashMap::new();
    let mut start = (0,0);
    for (y,line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {start = (x as i32,y as i32);}
            tiles.insert((x as i32,y as i32), c);
        }
    }
    let mut visited = HashSet::from([start]);
    let mut loop_border = HashSet::from([start]);
    loop {
        let next_border: HashSet<_> = loop_border.iter()
        .flat_map(|(x,y)| get_connected(*x, *y, tiles.clone()))
        .filter(|p| !visited.contains(p))
        .collect();
        loop_border = next_border;
        loop_border.iter().for_each(|p| {visited.insert(*p);});
        if loop_border.len() == 1 {break;}
    }

    let mut inside = 0;
    for y in 0..*visited.iter().map(|(_,y)| y).max().unwrap() {
        let mut currently_inside = false;
        for x in 0..*visited.iter().map(|(x,_)| x).max().unwrap() {
            if visited.contains(&(x,y)) && get_connected(x, y, tiles.clone()).contains(&(x,y+1)) {currently_inside = !currently_inside;}
            else if !visited.contains(&(x,y)) && currently_inside {inside += 1;}
        } 
    } 
    dbg!(inside);
}
fn get_connected(x:i32,y:i32, tiles: HashMap<(i32,i32), char>)-> HashSet<(i32,i32)> { 
    let N = (0,-1);
    let S = (0,1);
    let E = (1,0);
    let W = (-1,0);
    let directions = [N,S,E,W];
    let tile = tiles.get(&(x,y)).unwrap_or(&'.'); let connections = HashMap::from([
        ('|', [N,S]),
        ('-',[E,W]),
        ('L',[N,E]),
        ('J',[N,W]),
        ('7',[S,W]),
        ('F',[S,E]),
        ('.',[(0,0),(0,0)])
    ]);
    match tile {
        '.' => return HashSet::new(),
        'S' => return directions.iter()
        .map(|(dx,dy)| (x+dx,y+dy))
        .filter(|(ox,oy)| 
            get_connected(*ox, *oy,  tiles.clone())
            .contains(&(x,y)))
        .collect(),

        _ =>
        connections.get(tile).unwrap().iter()
        .map(|(dx, dy)| (x+dx,y+dy))
        .collect()
    }
}
