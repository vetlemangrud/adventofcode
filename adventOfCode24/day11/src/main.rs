use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut nums: Vec<i64> = input
        .split(" ")
        .map(|s| s.trim().parse().unwrap())
        .collect();
    for _ in 0..25 {
        let mut new_nums = Vec::new();
        for num in &nums {
            let digits = num.to_string().len();
            if *num == 0 {
                new_nums.push(1);
            } else if digits % 2 == 0 {
                let raise = 10.0_f64.powi(digits as i32 / 2);
                let a = f64::floor(*num as f64 / raise) as i64;
                let b = num - a * raise as i64;
                new_nums.push(a);
                new_nums.push(b);
            } else {
                new_nums.push(num * 2024);
            }
        }
        nums = new_nums;
    }
    dbg!(&nums.len());
}

fn part2(input: String) {
    let nums: Vec<i64> = input
        .split(" ")
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let mut map = HashMap::new();
    let f: u64 = nums.iter().map(|n| do_steps(*n as u64, 75, &mut map)).sum();
    dbg!(f);
}
fn do_steps(num: u64, steps: u64, map: &mut HashMap<(u64, u64), u64>) -> u64 {
    if steps == 0 {
        return 1;
    }
    match map.get(&(num, steps)) {
        Some(nums) => *nums,
        None => {
            let mut nums = Vec::new();
            let digits = num.to_string().len();
            if num == 0 {
                nums.push(1);
            } else if digits % 2 == 0 {
                let raise = 10.0_f64.powi(digits as i32 / 2);
                let a = f64::floor(num as f64 / raise) as u64;
                let b = num - a * raise as u64;
                nums.push(a);
                nums.push(b);
            } else {
                nums.push(num * 2024);
            }
            let final_nums: u64 = nums.iter().map(|n| do_steps(*n, steps - 1, map)).sum();
            map.insert((num, steps), final_nums);
            final_nums
        }
    }
}
