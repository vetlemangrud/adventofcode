use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::max;

fn calc_max_pressure(left: u32, elf: &str, elf_wait: u32, elephant: &str, elephant_wait: u32, flow_rate: &HashMap<&str, u32>, interesting_labels: &HashSet<&str> ,distance: &HashMap<(&str, &str), u32>, open: &HashSet<&str>) -> u32 {
    if left <= 0 {return 0};
    let both_wait = calc_max_pressure(left-1, elf, elf_wait+1, elephant, elephant_wait+1, flow_rate, interesting_labels, distance, open);

    let elf_move = interesting_labels.iter()
        .filter(|l| distance.get(&(elf, l)).unwrap() + 1 == elf_wait)
        .filter(|l| !open.contains(*l))
        .map(|l| {
            let mut open = open.clone();
            open.insert(l);
            flow_rate.get(*l).unwrap() * left + calc_max_pressure(left, l, 0, elephant, elephant_wait, flow_rate, interesting_labels, distance, &open)}).max().unwrap_or(0);
    
    let elephant_move = interesting_labels.iter()
        .filter(|l| distance.get(&(elephant, l)).unwrap() + 1 == elephant_wait)
        .filter(|l| !open.contains(*l))
        .map(|l| {
            let mut open = open.clone();
            open.insert(l);
            flow_rate.get(*l).unwrap() * left + calc_max_pressure(left, elf, elf_wait, l, 0, flow_rate, interesting_labels, distance, &open)}).max().unwrap_or(0);
    max(both_wait,max(elf_move, elephant_move))

}

fn main() {
    //Parse input
    let input = fs::read_to_string("input").unwrap();
    let mut labels: HashSet<&str> = HashSet::new();
    let mut flow_rate: HashMap<&str, u32> = HashMap::new();
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let parts = line.split(";").collect::<Vec<&str>>();
        let meta = parts[0].split(",").collect::<Vec<&str>>();
        let label = meta[0];
        labels.insert(label);
        flow_rate.insert(label, meta[1].parse().unwrap());
        let mut local_edges: Vec<&str> = vec![label];
        for edge in parts[1].split(",") {
            local_edges.push(edge);
        }
        edges.insert(label, local_edges);
    }

    // Create distance map between every vertex (Floyd-Warshall)
    let mut distance: HashMap<(&str, &str), u32> = HashMap::new();
    for i in &labels{
        for j in &labels {
            if i == j {
                distance.insert((i,j), 0);
            } 
            else if edges.get(i).unwrap().contains(j) {
                distance.insert((i,j), 1);
            }
            else {
                distance.insert((i,j), 99999999); //Big number
            }
        }
    }
    for i in &labels {
        for j in &labels {
            for k in &labels {
                if *distance.get(&(i,j)).unwrap() > distance.get(&(i,k)).unwrap() + distance.get(&(k,j)).unwrap() {
                   distance.insert((i,j),distance.get(&(i,k)).unwrap() + distance.get(&(k,j)).unwrap()); 
                }
            }
        }
    }

    let interesting_labels: HashSet<&str> = labels.iter()
        .map(|l| *l)
        .filter(|l| flow_rate.get(l).unwrap() > &0 || l == &"AA").collect();
    let open: HashSet<&str> = HashSet::new();
    dbg!(calc_max_pressure(26, "AA", 0, "AA", 0,  &flow_rate, &interesting_labels, &distance, &open));
    

}
