use std::fs;
use serde_json::Value;
use std::cmp;
use std::cmp::Ordering;


fn smart_split(to_split: &str) -> Vec<String> {
    let v: Value = serde_json::from_str(to_split).unwrap();
    
    v.as_array().unwrap()
        .iter()
        .map(|val| serde_json::to_string(val).unwrap())
        .collect()
        
}

fn check_order(left: &str, right: &str) -> Ordering {
    let left_is_list = left.chars().next().unwrap() == '[';
    let right_is_list = right.chars().next().unwrap() == '[';

    if !left_is_list && !right_is_list {
        let left: u32 = left.parse().unwrap();
        let right: u32 = right.parse().unwrap();
        if left < right {
            return Ordering::Less;
        } 
        else if left > right {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }

    if left_is_list && right_is_list {
        let left_list = smart_split(left);
        let right_list = smart_split(right);
        for i in 0..cmp::max(left_list.len(), right_list.len()) {
            if left_list.len() < i + 1 {
                return Ordering::Less;
            }
            else if right_list.len() < i +1 {
                return Ordering::Greater;
            }
            match check_order(&*left_list[i], &*right_list[i]) {
                Ordering::Equal =>(),
                other => return other 
            }
        }
        return Ordering::Equal;
    }

    if !left_is_list {
        return check_order(&*format!("[{}]", left), right);
    }
    if !right_is_list {
        return check_order(left, &*format!("[{}]", right));
    }


    return Ordering::Equal;

}

fn main() {


    let input = fs::read_to_string("input").unwrap();
    let mut lines: Vec<&str> = input.lines().filter(|line| line != &"").collect();
    let mut right_indicies = 0;
    for i in 0..(lines.len() / 2) {
       match check_order(lines[2*i], lines[2*i+1]) {
            Ordering::Less => {
                right_indicies += i + 1
            },
            _ => ()
       } 
    }
    dbg!(right_indicies);
    lines.push("[[2]]");
    lines.push("[[6]]");
    lines.sort_by(|a, b| check_order(a,b));

    let mut first_divider = 0;
    let mut second_divider = 0;
    for i in 0..lines.len() {
        if lines[i] == "[[2]]" {first_divider = i+1;}
        if lines[i] == "[[6]]" {second_divider = i+1;}
    }

    dbg!(first_divider * second_divider);
}
