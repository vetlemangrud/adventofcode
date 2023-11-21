use std::fs;
use std::collections::HashSet;

fn distSq(cube1: &(u32,u32,u32) ,cube2: &(u32,u32,u32)) -> u32 {
    (i32::pow(cube1.0 as i32 - cube2.0 as i32, 2) +
    i32::pow(cube1.1 as i32 - cube2.1 as i32, 2) +
    i32::pow(cube1.2 as i32 - cube2.2 as i32, 2)) as u32
}


fn main() {
    let input = fs::read_to_string("input").unwrap();
    let solid_cubes: HashSet<(u32,u32,u32)> = input.lines()
        .map(|l| {
            let nums = l.split(",")
                .map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            (nums[0], nums[1], nums[2])
        }
            ).collect();

    let mut possible_cubes: HashSet<(u32,u32,u32)> = HashSet::new();
    for x in 0..=20 {
        for y in 0..=20 {
            for z in 0..=20 {
                possible_cubes.insert((x,y,z));
            }
        }
    }

    let air_cubes: HashSet<(u32,u32,u32)> = possible_cubes.difference(&solid_cubes).map(|c| *c).collect();
    
    //Find all exterior cubes
    let mut frontier: HashSet<(u32,u32,u32)> = HashSet::new();
    let mut outside_cubes: HashSet<(u32,u32,u32)> = HashSet::new();
    frontier.insert((0,0,0));
    while frontier.len() > 0 {
        let curr = *frontier.iter().next().unwrap();
        outside_cubes.insert(curr);
        frontier.remove(&curr);
        for neighbor in air_cubes.iter()
            .filter(|c| !outside_cubes.contains(c))
            .filter(|c| distSq(&curr, c) == 1) {
                frontier.insert(*neighbor);        
            }
    }

    let not_outside_cubes: HashSet<(u32,u32,u32)> = possible_cubes.difference(&outside_cubes).map(|c| *c).collect();


    let mut surface = not_outside_cubes.len() * 6;
    for cube1 in &not_outside_cubes {
        for cube2 in not_outside_cubes.iter().filter(|c2| c2.0 + c2.1 *100 + c2.2 * 10000 < cube1.0 + cube1.1 *100 + cube1.2 * 10000) {
            if distSq(cube1, cube2) == 1 {
                surface -= 2;
            }
        }
    }
    dbg!(surface);

}
