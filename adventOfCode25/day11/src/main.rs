use memoize::memoize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    process::Output,
};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part1(input);
}

#[memoize(Ignore:outputs)]
fn paths_to_out(
    outputs: &HashMap<String, HashSet<String>>,
    input: String,
    visited_dac: bool,
    visited_fft: bool,
) -> u64 {
    if input == "out" {
        if visited_fft && visited_dac {
            return 1;
        }
        return 0;
    }
    let dac = visited_dac || input == "dac";
    let fft = visited_fft || input == "fft";

    let mut out = 0;
    for next in outputs.get(&input).unwrap() {
        out += paths_to_out(outputs, next.clone(), dac, fft);
    }

    out
}

fn part1(text: String) {
    let mut outputs: HashMap<String, HashSet<String>> = HashMap::new();
    for line in text.lines() {
        let string = line.to_string();
        let input = string[0..3].to_string();
        let out: HashSet<_> = string[5..]
            .to_string()
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        outputs.insert(input.clone(), out);
    }

    let out = paths_to_out(&outputs, "svr".to_string(), false, false);
    dbg!(&out);
}
