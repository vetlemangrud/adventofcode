use good_lp::{
    Expression, ProblemVariables, Solution, Solver, SolverModel, Variable, constraint,
    default_solver, variable, variables,
};
use itertools::Itertools;
use memoize::memoize;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::{self},
};

// This day got extremely messy as I'm trying different things

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}

fn next_state((lights, joltage): (u32, Vec<u32>), button: u32) -> (u32, Vec<u32>) {
    let mut new_joltage = joltage.clone();
    for i in 0..joltage.len() {
        if button & (1 << (joltage.len() - i - 1)) != 0 {
            new_joltage[i] += 1;
        }
    }
    (lights ^ button, new_joltage)
}

// fn part1(input: String) {
//     let mut out = 0;
//     let light_regex = Regex::new(r"\[(.*)\]").unwrap();
//     let joltage_regex = Regex::new(r"\{(.*)\}").unwrap();
//     let button_regex = Regex::new(r"\(((?:\d|,)*)\)").unwrap();
//     for line in input.lines() {
//         let light_capture = light_regex.captures(line).unwrap().get(1).unwrap().as_str();
//         let joltage_capture = joltage_regex
//             .captures(line)
//             .unwrap()
//             .get(1)
//             .unwrap()
//             .as_str();
//         let button_captures = button_regex.captures_iter(line);
//         let mut goal: u32 = 0;
//         let mut buttons: HashSet<u32> = HashSet::new();
//         let light_amount = light_capture.len();
//         let mut joltage_goal: Vec<u32> = Vec::new();
//         for char in light_capture.chars() {
//             goal = goal << 1;
//             match char {
//                 '.' => (),
//                 '#' => goal = goal | 1,
//                 _ => panic!("invalid light"),
//             }
//         }
//         for joltage in joltage_capture.split(",") {
//             joltage_goal.push(joltage.parse().unwrap());
//         }
//         for button_capture in button_captures {
//             buttons.insert(
//                 button_capture
//                     .get(1)
//                     .unwrap()
//                     .as_str()
//                     .split(",")
//                     .map(|n| 1 << (light_amount - n.parse::<usize>().unwrap() - 1))
//                     .fold(0, |a, v| a | v),
//             );
//         }
//         let mut states: HashSet<u32> = HashSet::new();
//         states.insert(0);
//         let mut presses = 0;
//         loop {
//             presses += 1;
//             states = states
//                 .clone()
//                 .iter()
//                 .flat_map(|s| buttons.iter().map(|b| next_state(*s, *b)))
//                 .collect();
//             if states.contains(&goal) {
//                 break;
//             }
//         }
//         out += presses;
//     }
//     dbg!(out);
// }

// Did not work
#[memoize(Ignore:buttons)]
fn is_possible(
    buttons: &Vec<HashSet<usize>>,
    line: usize,
    buttons_taken: usize,
    presses_left: i64,
    jolt_goal: Vec<i64>,
) -> bool {
    if presses_left == 0 {
        return false;
    }
    if buttons_taken == buttons.len() {
        return false;
    }
    if jolt_goal.iter().all(|v| *v == 0) {
        return true;
    }
    if jolt_goal.iter().any(|v| *v > presses_left) {
        return false;
    }
    if jolt_goal.iter().any(|v| *v < 0) {
        return false;
    }
    if jolt_goal.iter().enumerate().any(|(goal_i, goal_v)| {
        goal_v > &0
            && !buttons
                .iter()
                .skip(buttons_taken)
                .any(|button| button.contains(&goal_i))
    }) {
        return false;
    }
    if jolt_goal.iter().fold(0, |a, b| a + b)
        > presses_left
            * buttons
                .iter()
                .skip(buttons_taken)
                .map(|b| b.len())
                .max()
                .unwrap() as i64
    {
        return false;
    }
    (0..=presses_left).any(|first_button_presses| {
        let mut new_jolt_goal = jolt_goal.clone();
        let button = buttons.get(buttons_taken).unwrap();
        for wire in button {
            new_jolt_goal[*wire] -= first_button_presses;
        }
        is_possible(
            buttons,
            line,
            buttons_taken + 1,
            presses_left - first_button_presses,
            new_jolt_goal,
        )
    })
}

fn solve_part2_line(buttons: Vec<HashSet<usize>>, joltage_goal: Vec<u32>) -> f64 {
    let mut vars = variables!();
    let mut var_vec: Vec<(HashSet<usize>, Variable)> = Vec::new();
    for button in buttons {
        var_vec.push((button, vars.add(variable().min(0).integer())));
    }

    let objective: Expression = var_vec.iter().map(|(_, v)| v).sum();
    let mut p = vars.minimise(objective.clone()).using(default_solver);

    for (i, v) in joltage_goal.iter().enumerate() {
        let expression: Expression = var_vec
            .iter()
            .filter(|(b, _)| b.contains(&i))
            .map(|(_, v)| v)
            .sum();

        p.add_constraint(constraint!(expression == *v));
    }
    let solution = p.solve().unwrap();
    let answer = solution.eval(&objective);
    dbg!(answer);
    answer
}

fn part2(input: String) {
    let mut out: f64 = 0.0;
    let joltage_regex = Regex::new(r"\{(.*)\}").unwrap();
    let button_regex = Regex::new(r"\(((?:\d|,)*)\)").unwrap();
    let total_lines = input.lines().count();
    for (i, line) in input.lines().enumerate() {
        println!("{}/{}", &i + 1, &total_lines);
        let joltage_capture = joltage_regex
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let button_captures = button_regex.captures_iter(line);
        let mut buttons: Vec<HashSet<usize>> = Vec::new();
        let mut joltage_goal: Vec<u32> = Vec::new();
        for joltage in joltage_capture.split(",") {
            joltage_goal.push(joltage.parse().unwrap());
        }
        for button_capture in button_captures {
            buttons.push(
                button_capture
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(",")
                    .map(|b| b.parse().unwrap())
                    .collect(),
            );
        }
        out += solve_part2_line(buttons, joltage_goal);
    }
    dbg!(&out);
}
