use std::fs;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part1(input);
}
fn part1(input: String) {
    let result:i64= input.lines().map(snafu_to_int).sum();
    let snafu = int_to_snafu(result);
    dbg!(snafu);
}

fn snafu_to_int(snafu: &str) -> i64 {
    let mut total = 0;
    for (i, c) in snafu.chars().enumerate() {
        let p = (snafu.len()-i-1) as u32;
        total += match c {
            '2' => 2,
            '1' => 1,
            '-' => -1,
            '=' => -2,
            _ => 0
        } * 5_i64.pow(p);
    }
    total
}

fn int_to_snafu(int: i64) -> String {
    let num_symbols = ((int as f64).log(5.0).ceil() as i64);
    let mut left = int;
    let mut out = String::new();
    for i in 0..num_symbols {
       let tail = "2".repeat((num_symbols-1-i) as usize); 
       if snafu_to_int((String::from("1")+tail.as_str()).as_str()) < left {
           out += "2";
           left -= 2 * 5_i64.pow((num_symbols-i-1)as u32);
       } 
       else if snafu_to_int((String::from("0")+tail.as_str()).as_str()) < left {
           out += "1";
           left -= 1 * 5_i64.pow((num_symbols-i-1)as u32);
       } 
       else if snafu_to_int((String::from("-")+tail.as_str()).as_str()) < left {
           out += "0";
       } 
       else if snafu_to_int((String::from("=")+tail.as_str()).as_str()) < left {
           out += "-";
           left += 1 * 5_i64.pow((num_symbols-i-1)as u32);
       } else {

           out += "=";
           left += 2 * 5_i64.pow((num_symbols-i-1)as u32);
       }
    }
    out
}