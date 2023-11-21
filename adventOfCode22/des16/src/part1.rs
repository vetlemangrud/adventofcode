use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::{max,min};

fn calc_max_pressure(left: u32, label: &str, flow_rate: &HashMap<&str, u32>, interesting_labels: &HashSet<&str> ,distance: &HashMap<(&str, &str), u32>, open: &HashSet<&str>) -> u32 {
    if left <= 0 {return 0};
    if open.contains(label) {0} 
    else {
        let mut open = open.clone();
        open.insert(label);
        left * flow_rate.get(label).unwrap() + 
            interesting_labels.iter()
            .filter(|l| l != &&label)
            .map(|l| calc_max_pressure(left - min(left, 1 + distance.get(&(label,l)).unwrap()), l, flow_rate, interesting_labels, distance, &open)).max().unwrap()
    }

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
    dbg!(calc_max_pressure(30, "AA", &flow_rate, &interesting_labels, &distance, &open));
    

}
