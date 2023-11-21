use std::fs;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut stacks: Vec<Vec<char>> = input.lines().take(9).map(|line| 
        line.chars().collect::<Vec<char>>()
    ).collect();
    let instructions = input.lines()
        .skip(10)
        .map(|line| line
             .split(",")
             .map(|char| char.parse::<usize>().unwrap())
             .collect::<Vec<usize>>()
             );
    for ins in instructions {
        dbg!(&ins);
        let mut moving: Vec<char> = Vec::new();
        for _ in 0..ins[0] {
            moving.push(*(&stacks[ins[1]-1].pop().unwrap()));
        }
        for _ in 0..ins[0] {
            stacks[ins[2]-1].push(moving.pop().unwrap());
        }
        
        dbg!(&stacks);
    }

}
