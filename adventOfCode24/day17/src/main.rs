use std::{collections::HashMap, fs, hash::BuildHasher, thread};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut regs = re
        .captures_iter(&input)
        .map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap());
    let mut a = regs.next().unwrap();
    let mut b = regs.next().unwrap();
    let mut c = regs.next().unwrap();
    let program: Vec<_> = regs.collect();
    let mut pc = 0;
    let mut output = Vec::new();
    while pc < program.len() {
        // dbg!(pc);
        // dbg!((a, b, c));
        let opcode = program[pc];
        let literal = program[pc + 1];
        let combo = match literal {
            4 => a,
            5 => b,
            6 => c,
            _ => literal,
        };

        match opcode {
            0 => {
                //adv
                a = a / 2_i32.pow(combo as u32);
            }
            1 => {
                //bxl

                b = b ^ literal;
            }
            2 => {
                //bst
                b = combo % 8;
            }
            3 => {
                //jnz
                if a != 0 {
                    pc = literal as usize;
                }
            }
            4 => {
                //bxc
                b = b ^ c;
            }
            5 => {
                //out
                output.push(combo % 8);
            }
            6 => {
                //bdv
                b = a / 2_i32.pow(combo as u32);
            }
            _ => {
                //cdv
                c = a / 2_i32.pow(combo as u32);
            }
        }

        if opcode != 3 || a == 0 {
            pc += 2
        };
    }
    let print: Vec<_> = output.iter().map(|v| v.to_string()).collect();
    dbg!(print.join(","));
}

fn part2(input: String) {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut regs = re
        .captures_iter(&input)
        .map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap());
    let _a = regs.next().unwrap();
    let _b = regs.next().unwrap();
    let _c = regs.next().unwrap();
    let program: Vec<_> = regs.collect();
    let mut correct = 0;
    let mut a = 0;
    dbg!(get_output(203146982337066));
    loop {
        let output = get_output(a);
        if (0..correct + 1).all(|i| output.get(i) == program.get(program.len() - correct - 1 + i)) {
            dbg!(correct);
            dbg!(a);
            correct += 1;
            if correct == program.len() {
                break;
            }
            a *= 8;
            a -= 8;
        }
        a += 1;
    }
    //4445818 too low
    // Most probably higher than 3e13
    // 203146982337066 is too high
} //Program: 2,4,1,1,7,5,0,3,1,4,4,5,5,5,3,0
  // b = a % 8
  // b = b ^ 1
  // c = a / 2 ** b
  // a = a / 2 ** 3
  // b = b ^ 4
  // b = b ^ c
  // out(b % 8)
  // a > 0: jmp 0

fn get_output(a: i64) -> Vec<i32> {
    let mut a = a;
    let mut b;
    let mut c;
    let mut output = Vec::new();
    while a != 0 {
        b = a % 8;
        b = b ^ 1;
        c = a / 2_i64.pow(b as u32);
        a = a / 2_i64.pow(3 as u32);
        b = b ^ 4;
        b = b ^ c;
        output.push((b % 8) as i32);
    }
    output
}

fn get_output_old(
    a: i32,
    b: i32,
    c: i32,
    program: &Vec<i32>,
    map: &mut HashMap<(i32, i32, i32), Vec<i32>>,
) -> Vec<i32> {
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut pc = 0;
    let mut output = Vec::new();
    if map.contains_key(&(a, b, c)) {
        let mut next_out = map.get_mut(&(a, b, c)).unwrap().to_vec();
        output.append(&mut next_out);
    }
    while pc < program.len() {
        let opcode = program[pc];
        let literal = program[pc + 1];
        let combo = match literal {
            4 => a,
            5 => b,
            6 => c,
            _ => literal,
        };

        //Register A: 51571418
        //Register B: 0
        //Register C: 0

        match opcode {
            0 => {
                //adv
                a = a / 2_i32.pow(combo as u32);
            }
            1 => {
                //bxl

                b = b ^ literal;
            }
            2 => {
                //bst
                b = combo % 8;
            }
            3 => {
                //jnz
                if a != 0 {
                    pc = literal as usize;
                }
            }
            4 => {
                //bxc
                b = b ^ c;
            }
            5 => {
                //out
                output.push(combo % 8);
                if (0..output.len()).any(|i| output.get(i) != program.get(i)) {
                    break;
                }
            }
            6 => {
                //bdv
                b = a / 2_i32.pow(combo as u32);
            }
            _ => {
                //cdv
                if 2_i32.pow(combo as u32) == 0 {
                    break;
                }
                c = a / 2_i32.pow(combo as u32);
            }
        }

        if opcode != 3 || a == 0 {
            pc += 2
        };
    }
    output
}
