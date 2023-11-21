use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut height_map: HashMap<(i32,i32), u32> = HashMap::new();
    let mut parent_map: HashMap<(i32,i32), (i32,i32)> = HashMap::new();
    let mut start: (i32,i32) = (0,0);
    let mut end: (i32,i32) = (0,0);
    let height = input.lines().collect::<Vec<&str>>().len() as i32;
    let width = input.lines().next().unwrap().chars().collect::<Vec<char>>().len() as i32;

    //Populate height_map
    for (ys, line) in input.lines().enumerate() {
        for (xs, c) in line.chars().enumerate() {
            let y:i32 = ys.try_into().unwrap();
            let x:i32 = xs.try_into().unwrap();

            if c == 'S' {
                start = (x, y);
                height_map.insert((x,y), 0);
            }
            else if c == 'E' {
                end = (x, y);
                height_map.insert((x,y), 25);
            }
            else {
                let height = c as u32 - 97;
                height_map.insert((x,y), height);
            }
        }
    }

    //Implement Dijkstras alogrithm
    let directions: Vec<(i32, i32)> = vec![(1,0),(0,-1),(-1,0),(0,1)];
    let mut frontier_queue = VecDeque::from([end]);
    while frontier_queue.len() > 0 {
        let current = frontier_queue.pop_front().unwrap();
        if height_map.get(&current).unwrap() == &0 {
            start = current;
            break;
        }
        for direction in &directions {
            let neighbor = (current.0 + direction.0, current.1 + direction.1);
            if neighbor.0 < 0 || 
            neighbor.0 >= width || 
            neighbor.1 < 0 || 
            neighbor.1 >= height {
                continue;
            }
            if neighbor == end || parent_map.contains_key(&neighbor) {
                continue;
            }
            if height_map.get(&neighbor).unwrap() < &(height_map.get(&current).unwrap()-1) {
            continue;
            }
            parent_map.insert(neighbor, current);
            frontier_queue.push_back(neighbor);
        }
    }


    //Backtrack to find length
    dbg!(height_map.get(&start));
    let mut current = start;
    let mut length = 0;
    while current != end {
        current = *parent_map.get(&current).unwrap();
        length += 1;
    }
    dbg!(length);
}
