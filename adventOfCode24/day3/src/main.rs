use regex::Regex;
use std::fs;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn part1(input: String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    for cap in re.captures_iter(&input) {
        total += cap.get(1).unwrap().as_str().parse::<i32>().unwrap()
            * cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
    }
    total
}

fn part2(input: String) {
    let input_with_explicit_do = String::from("do()") + &input + "don't()";
    let re = Regex::new(r"do\(\)((?s).*?)don't\(\)").unwrap();
    let total = re
        .captures_iter(&input_with_explicit_do)
        .map(|m| String::from(m.get(1).unwrap().as_str()))
        .map(|s| part1(s))
        .reduce(|acc, s| acc + s)
        .unwrap();
    println!("{}", total)
}
