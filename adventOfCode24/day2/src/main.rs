use std::{fs, iter::zip};
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn part1(input: String) {
    let mut safe = 0;
    for line in input.lines() {
        let mut report: Vec<i32> = Vec::new();
        for level in line.split(" ") {
            report.push(level.parse().unwrap());
        }
        let pairs = zip(report.clone(), report.iter().skip(1));

        // Check if monotone
        let comparisons = pairs.clone().map(|(a, b)| a.cmp(b));
        let monotone = comparisons.clone().min() == comparisons.clone().max();
        if !monotone {
            continue;
        }

        // Check step sizes
        let legal_step_sizes = pairs.map(|(a, b)| (a - b).abs()).all(|s| s >= 1 && s <= 3);
        if !legal_step_sizes {
            continue;
        }
        safe += 1;
    }
    println!("{}", safe);
}

fn part2(input: String) {
    let mut safe = 0;
    for line in input.lines() {
        let mut report: Vec<i32> = Vec::new();
        for level in line.split(" ") {
            report.push(level.parse().unwrap());
        }

        let pairs = zip(report.clone(), report.iter().skip(1));

        // Check if monotone
        let comparisons = pairs.clone().map(|(a, b)| a.cmp(b));
        let monotone = comparisons.clone().min() == comparisons.clone().max();

        // Check step sizes
        let legal_step_sizes = pairs.map(|(a, b)| (a - b).abs()).all(|s| s >= 1 && s <= 3);
        if monotone && legal_step_sizes {
            safe += 1;
            continue;
        }

        // If not safe, check all variations (Nasty)
        if (0..report.len()).any(|i| {
            let mut rep_var = report.clone();
            rep_var.remove(i);
            dbg!(&rep_var);
            let pairs = zip(rep_var.clone(), rep_var.iter().skip(1));

            // Check if monotone
            let comparisons = pairs.clone().map(|(a, b)| a.cmp(b));
            let monotone = comparisons.clone().min() == comparisons.clone().max();

            // Check step sizes
            let legal_step_sizes = pairs.map(|(a, b)| (a - b).abs()).all(|s| s >= 1 && s <= 3);

            return monotone && legal_step_sizes;
        }) {
            safe += 1;
        }
    }
    println!("{}", safe);
}
