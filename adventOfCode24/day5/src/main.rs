use std::{cmp::Ordering, collections::HashSet, fs};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let rule_string = input.split("\n\n").next().unwrap();
    let mut rules = HashSet::<(i32, i32)>::new();
    for rule in rule_string.lines() {
        let mut nums = rule.split("|");
        let num1: i32 = nums.next().unwrap().parse().unwrap();
        let num2: i32 = nums.next().unwrap().parse().unwrap();
        rules.insert((num1, num2));
    }

    let mut total = 0;

    let page_inputs = input.split("\n\n").skip(1).next().unwrap();
    for page_input in page_inputs.lines() {
        let pages = page_input.split(",").map(|n| n.parse::<i32>().unwrap());
        let res = pages.fold(Some(Vec::new()), |prev, new| match prev {
            None => None,
            Some(nums) => {
                if nums
                    .clone()
                    .iter()
                    .any(|n: &i32| rules.contains(&(new, *n)))
                {
                    None
                } else {
                    let mut mod_vec: Vec<_> = nums.clone();
                    mod_vec.push(new);
                    Some(mod_vec)
                }
            }
        });

        match res {
            None => (),
            Some(nums) => total += nums.get(nums.len().div_ceil(2) - 1).unwrap(),
        }
    }
    dbg!(total);
}

fn part2(input: String) {
    let rule_string = input.split("\n\n").next().unwrap();
    let mut rules = HashSet::<(i32, i32)>::new();
    for rule in rule_string.lines() {
        let mut nums = rule.split("|");
        let num1: i32 = nums.next().unwrap().parse().unwrap();
        let num2: i32 = nums.next().unwrap().parse().unwrap();
        rules.insert((num1, num2));
    }

    let mut total = 0;

    let page_inputs = input.split("\n\n").skip(1).next().unwrap();
    for page_input in page_inputs.lines() {
        let pages = page_input.split(",").map(|n| n.parse::<i32>().unwrap());
        let res = pages
            .clone()
            .fold(Some(Vec::new()), |prev, new| match prev {
                None => None,
                Some(nums) => {
                    if nums
                        .clone()
                        .iter()
                        .any(|n: &i32| rules.contains(&(new, *n)))
                    {
                        None
                    } else {
                        let mut mod_vec: Vec<_> = nums.clone();
                        mod_vec.push(new);
                        Some(mod_vec)
                    }
                }
            });

        if res.is_some() {
            continue;
        }

        let mut page_vec: Vec<i32> = pages.collect();
        page_vec.sort_by(|a, b| {
            if rules.contains(&(*a, *b)) {
                Ordering::Less
            } else if rules.contains(&(*b, *a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        total += page_vec.get(page_vec.len().div_ceil(2) - 1).unwrap();
    }
    dbg!(total);
}
