use std::fs;
use std::io;
use std::io::Write;
fn main() {
    
    let input = fs::read_to_string("input").unwrap();


    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    let mut drawn_pixels: Vec<i32> = Vec::new();

    for line in input.lines() {
        let split_line: Vec<&str> = line.split(" ").collect();
        let opcode = split_line[0];
        if opcode == "noop" {
            if ((cycle % 40) - x).abs() <= 1 {
                drawn_pixels.push(cycle);
            }
            cycle += 1;
        }
        else if opcode == "addx" {
            if ((cycle % 40) - x).abs() <= 1 {
                drawn_pixels.push(cycle);
            }
            if (((cycle+1) % 40) - x).abs() <= 1 {
                drawn_pixels.push(cycle+1);
            }
            cycle += 2;
            x += split_line[1].parse::<i32>().unwrap();
        }
        
    }
    for row in 0..8 {
        for col in 0..40 {
            if drawn_pixels.contains(&(40 * row + col)) {
                print!("X");
            } else {
                print!(" ");
            }
            io::stdout().flush().unwrap();
        }
        println!("");
    }

}
