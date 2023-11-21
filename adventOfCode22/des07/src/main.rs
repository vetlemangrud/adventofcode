use std::fs;
use std::collections::HashMap;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut path: Vec<&str> = Vec::new();
    let mut directories: HashMap<String, u32> = HashMap::new();
    let mut checking: bool = false;
    let mut listed: Vec<String> = Vec::new();

    for line in input.lines() {
        let split_line: Vec<&str> = line.split(" ").collect();
        if split_line[0] == "$" && checking {
            listed.push(path.join("/"));
            checking = false;
        }

        if split_line[0] == "$" && split_line[1] == "cd" {
            if split_line[2] == ".." {
                path.pop();
                continue;
            } else if split_line[2] == "/" {
                path.clear();
                path.push("/");
                continue;
            } else {
                path.push(split_line[2]);
                continue;
            }
        } else if split_line[0] == "$" && split_line[1] == "ls" {
            if !listed.contains(&path.join("/")) {
                checking = true;
            }
            else {
                dbg!(&path);
            }
        } else if split_line[0] != "dir" && checking {
            let size: u32 = split_line[0].parse().unwrap(); 
            for i in 1..=path.len() {
                let p = path.iter().take(i).map(|s| *s).collect::<Vec<&str>>().join("/");
                directories.entry(p).and_modify(|s| *s += size).or_insert(size);
            }
        }
    }

    let mut sum = 0;
    let to_delete = 913445;
    let mut best = 999999999;
    for (_, size) in directories.into_iter() {
        if size <= 100000 {
            sum += size;
        }
        if size >= to_delete && size <= best{
            best = size;
        }
    }
    dbg!(sum);
    dbg!(best);
    
}
