use std::collections::HashMap;
use std::fs;

fn line_to_score(line: &str, shape_score: &HashMap<&str, i32>) -> i32 {
    if line.chars().count() == 0 {
        return 0;
    }
    let shape_scores: Vec<&i32> = line.split(" ").map(|m| shape_score.get(m).unwrap()).collect();

    let result = **shape_scores.get(1).unwrap() * 3 + (**shape_scores.get(0).unwrap() + **shape_scores.get(1).unwrap() + 1) % 3 + 1;
    //dbg!(line);
    //dbg!(result);
    result
}


fn main() {
    let mut shape_score = HashMap::new();
    shape_score.insert("A", 1);
    shape_score.insert("B", 2);
    shape_score.insert("C", 3);
    shape_score.insert("X", 0);
    shape_score.insert("Y", 1);
    shape_score.insert("Z", 2);

    let file_path = "input";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    dbg!(contents.split("\n").map(|line| line_to_score(line, &shape_score)).sum::<i32>());
    
}
