use std::fs;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut numbers: Vec<(i64,usize)> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        numbers.push((line.parse::<i64>().unwrap() * 811589153,i));
    }
    for _ in 0..10 {
        for move_no in 0..numbers.len() {
            let mut new_numbers = numbers.clone();
            for (i, num) in numbers.iter().enumerate() {
                if num.1 != move_no {
                   continue; 
                }
                new_numbers.remove(i);
                let new_index = ((i as i64 + num.0).rem_euclid(new_numbers.len() as i64)) as usize;
                new_numbers.insert(new_index, *num);
                break;
            }
            numbers = new_numbers;
        }
    }

    let mut zero_i = 0;
    for (i, num) in numbers.iter().enumerate() {
        if num.0 == 0 {
            zero_i = i;
        }
    }

    let num1000 = numbers[(zero_i + 1000) % numbers.len()].0;
    let num2000 = numbers[(zero_i + 2000) % numbers.len()].0;
    let num3000 = numbers[(zero_i + 3000) % numbers.len()].0;
    dbg!(num1000 + num2000 + num3000);

    
}
