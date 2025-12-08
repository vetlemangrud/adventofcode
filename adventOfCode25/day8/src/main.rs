use std::{
    collections::{HashMap, HashSet},
    fs::{self},
};

use itertools::Itertools;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut positions: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut pos_to_circuit: HashMap<(i64, i64, i64), i64> = HashMap::new();
    let mut circuit_to_pos: HashMap<i64, HashSet<(i64, i64, i64)>> = HashMap::new();
    for line in input.lines() {
        let mut coordssplit = line.split(",").map(|c| c.parse::<i64>().unwrap());
        let coords = (
            coordssplit.next().unwrap(),
            coordssplit.next().unwrap(),
            coordssplit.next().unwrap(),
        );
        positions.insert(coords);
        let circuit_id = pos_to_circuit.len() as i64;
        pos_to_circuit.insert(coords, circuit_id);
        circuit_to_pos.insert(circuit_id, HashSet::from([(coords)]));
    }
    let mut combinations = positions
        .iter()
        .tuple_combinations::<(_, _)>()
        .sorted_by_key(|(a, b)| (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2));
    for _ in 0..1000 {
        let combination = combinations.next().unwrap();
        let old_circuit = *pos_to_circuit.get(combination.1).unwrap();
        let new_circuit = *pos_to_circuit.get(combination.0).unwrap();
        if old_circuit == new_circuit {
            continue;
        }
        let old_circuit_positions = circuit_to_pos.get_mut(&old_circuit).unwrap().clone();
        for coord in old_circuit_positions {
            pos_to_circuit.insert(coord, new_circuit);
            circuit_to_pos.get_mut(&new_circuit).unwrap().insert(coord);
        }
        circuit_to_pos.remove(&old_circuit);
    }
    let mut nodecount = circuit_to_pos.iter().map(|cir| cir.1.len()).sorted().rev();
    let mut out = 1;
    for _ in 0..3 {
        out *= nodecount.next().unwrap();
    }
    dbg!(out);
    // 399748 is too high
}
fn part2(input: String) {
    let mut positions: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut pos_to_circuit: HashMap<(i64, i64, i64), i64> = HashMap::new();
    let mut circuit_to_pos: HashMap<i64, HashSet<(i64, i64, i64)>> = HashMap::new();
    for line in input.lines() {
        let mut coordssplit = line.split(",").map(|c| c.parse::<i64>().unwrap());
        let coords = (
            coordssplit.next().unwrap(),
            coordssplit.next().unwrap(),
            coordssplit.next().unwrap(),
        );
        positions.insert(coords);
        let circuit_id = pos_to_circuit.len() as i64;
        pos_to_circuit.insert(coords, circuit_id);
        circuit_to_pos.insert(circuit_id, HashSet::from([(coords)]));
    }
    let mut combinations = positions
        .iter()
        .tuple_combinations::<(_, _)>()
        .sorted_by_key(|(a, b)| (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2));
    loop {
        let combination = combinations.next().unwrap();
        let old_circuit = *pos_to_circuit.get(combination.1).unwrap();
        let new_circuit = *pos_to_circuit.get(combination.0).unwrap();
        if old_circuit == new_circuit {
            continue;
        }
        let old_circuit_positions = circuit_to_pos.get_mut(&old_circuit).unwrap().clone();
        for coord in old_circuit_positions {
            pos_to_circuit.insert(coord, new_circuit);
            circuit_to_pos.get_mut(&new_circuit).unwrap().insert(coord);
        }
        circuit_to_pos.remove(&old_circuit);
        if circuit_to_pos.len() == 1 {
            dbg!(combination.0.0 * combination.1.0);
            break;
        }
    }
}
