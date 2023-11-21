use std::fs;

fn get_elf_cal(elf: &str) -> i32 {
    let calories: Vec<i32> = elf.split("\n").map(|item| item.parse::<i32>().unwrap_or(0)).collect();
    let calorie_sum: i32 = calories.iter().sum();
    dbg!(calorie_sum);
    calorie_sum

}


fn main() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let elves = contents.split("\n\n");
    
    let mut calories: Vec<i32> = elves.map(|elf| get_elf_cal(&elf)).collect();
    calories.sort();
    let sum: i32 = calories.iter().rev().take(3).sum();
    dbg!(sum);
}
