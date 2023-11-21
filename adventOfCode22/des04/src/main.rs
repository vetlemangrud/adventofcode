use std::fs;

fn contains(x1:u32,x2:u32,y1:u32,y2:u32) -> u32 {
    if !(x1 > y2 || x2 < y1) {
        1
    } else {
        0
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let lines = input.lines();
    let splitLines = lines.map(|line| line.split(",").map(|range| range.split("-").map(|no| no.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>()).collect::<Vec<Vec<Vec<u32>>>>();
    let result: u32 = splitLines.iter().map(|l| contains(l[0][0], l[0][1], l[1][0], l[1][1])).sum();
    dbg!(result);

}
