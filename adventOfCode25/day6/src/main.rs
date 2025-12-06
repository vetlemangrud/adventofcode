use std::{
    collections::HashMap,
    ffi::c_double,
    fs::{self, canonicalize},
    thread::current,
};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn part1(input: String) {
    let mut lines: Vec<&str> = input.lines().into_iter().collect();
    let operations: Vec<&str> = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim())
        .collect();

    let mut cells: HashMap<(i64, i64), i64> = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    for line in lines {
        x = 0;
        for cell in line.split_whitespace() {
            cells.insert((x, y), cell.parse().unwrap());
            x += 1;
        }
        y += 1;
    }
    let width = x;
    let height = y;
    let mut total = 0;
    for col_i in 0..width {
        let operation = operations.get(col_i as usize).unwrap();
        let mut col_total = match operation {
            &"*" => 1,
            _ => 0,
        };

        for row_i in 0..height {
            match operation {
                &"*" => col_total *= cells.get(&(col_i, row_i)).unwrap(),
                _ => col_total += cells.get(&(col_i, row_i)).unwrap(),
            }
        }
        total += col_total;
    }

    dbg!(total);
}

fn get_digit(num: i64, pos: i64) -> i64 {
    return num / ((10 as i64).pow(pos as u32)) - (num / ((10 as i64).pow(pos as u32 + 1))) * 10;
}

fn part2(input: String) {
    let mut lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut col_widths = Vec::<i32>::new();
    let mut operation_chars = lines.pop().unwrap().into_iter();
    let mut operations: Vec<char> = Vec::new();
    operations.push(operation_chars.next().unwrap());
    let mut current_col_width = 0;
    for c in operation_chars {
        if c == ' ' {
            current_col_width += 1;
        } else {
            operations.push(c);
            col_widths.push(current_col_width);
            current_col_width = 0;
        }
    }
    col_widths.push(current_col_width + 1);
    let mut total = 0;

    for _ in 0..operations.len() {
        let current_operation = operations.pop().unwrap();
        let current_col_width = col_widths.pop().unwrap();
        let mut current_total: i64 = if current_operation == '*' { 1 } else { 0 };
        for _ in 0..current_col_width {
            let mut next_num = String::new();
            for line in lines.iter_mut() {
                next_num.push(line.pop().unwrap());
            }
            let num: i64 = next_num.trim().parse().unwrap();
            if current_operation == '*' {
                current_total *= num;
            } else {
                current_total += num;
            }
        }
        total += current_total;

        for line in lines.iter_mut() {
            line.pop();
        }
    }
    dbg!(total);
}
