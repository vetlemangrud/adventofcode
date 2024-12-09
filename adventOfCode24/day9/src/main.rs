use std::{collections::VecDeque, fs, usize};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut blocks: Vec<i64> = Vec::new();
    let mut free_indexes = VecDeque::new();
    let mut filled_indexes = Vec::new();
    let mut id = 0;
    for (i, c) in input.chars().filter(|c| *c != '\n').enumerate() {
        if i % 2 == 0 {
            for _ in 0..c.to_digit(10).unwrap() {
                filled_indexes.push(blocks.len());
                blocks.push(id);
            }
            id += 1;
        } else {
            for _ in 0..c.to_digit(10).unwrap() {
                free_indexes.push_back(blocks.len());
                blocks.push(-1);
            }
        }
    }

    loop {
        let next_free = free_indexes.pop_front().unwrap();
        let next_filled = filled_indexes.pop().unwrap();
        if next_free < next_filled {
            blocks[next_free] = blocks[next_filled];
        } else {
            dbg!((next_free, next_filled));
            blocks.resize(next_filled + 1, -1);
            break;
        }
    }
    let checksum: i64 = blocks.iter().enumerate().map(|(i, b)| i as i64 * b).sum();
    dbg!(checksum);
    // 6384018292784 too low
}
fn part1_retry(input: String) {
    // More brute force, but it worked
    let mut blocks: Vec<Option<i64>> = Vec::new();
    let mut id = 0;
    for (i, c) in input.chars().filter(|c| *c != '\n').enumerate() {
        if i % 2 == 0 {
            for _ in 0..c.to_digit(10).unwrap() {
                blocks.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..c.to_digit(10).unwrap() {
                blocks.push(None);
            }
        }
    }

    loop {
        let next_free = blocks
            .iter()
            .enumerate()
            .filter(|(_i, v)| v.is_none())
            .map(|(i, _v)| i)
            .next()
            .unwrap();
        let next_filled = blocks
            .iter()
            .enumerate()
            .rev()
            .filter(|(_i, v)| v.is_some())
            .map(|(i, _v)| i)
            .next()
            .unwrap();
        if next_free < next_filled {
            blocks[next_free] = blocks[next_filled];
            blocks[next_filled] = None;
        } else {
            break;
        }
    }
    let checksum: i64 = blocks
        .iter()
        .enumerate()
        .filter(|(_i, b)| b.is_some())
        .map(|(i, b)| i as i64 * b.unwrap())
        .sum();
    dbg!(checksum);
}
fn part2(input: String) {
    let mut blocks = Vec::new();
    let mut free: Vec<(usize, u32)> = Vec::new();
    let mut id = 0;
    let mut loc: usize = 0;
    for (i, c) in input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
    {
        if i % 2 == 0 {
            blocks.push((id, loc, c));
            loc += c as usize;
            id += 1;
        } else {
            free.push((loc, c));
            loc += c as usize;
        }
    }

    let mut new_blocks: Vec<(i32, usize, u32)> = Vec::new();

    for (id, start, length) in blocks.iter().rev() {
        let free_spot = free
            .iter_mut()
            .enumerate()
            .filter(|(_, (s, _l))| s < start)
            .filter(|(_, (_s, l))| l >= length)
            .next();
        match free_spot {
            None => new_blocks.push((*id, *start, *length)),
            Some((i, range)) => {
                new_blocks.push((*id, range.0, *length));
                free[i] = (range.0 + *length as usize, range.1 - length);
            }
        }
    }
    new_blocks.sort_by_key(|(_, i, _)| *i);
    let sum: i64 = new_blocks
        .iter()
        .map(|(id, start, length)| {
            let mults = (*start as i64..*start as i64 + *length as i64).sum::<i64>();
            *id as i64 * mults
        })
        .sum();
    dbg!(sum);
}
