use std::fs;
use eval::{Expr, to_value};
const NO_ROUNDS:u32 = 10000;
const NO_MONKEYS:usize = 8;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut items: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<&str> = Vec::new();
    let mut test_numbers: Vec<u64> = Vec::new();
    let mut true_throws: Vec<usize> = Vec::new();
    let mut false_throws: Vec<usize> = Vec::new();
    let mut inspects: Vec<u32> = Vec::new();
    
    let mut lines = input.lines();
    for _ in 0..NO_MONKEYS {
        let monkey_items = lines.next().unwrap().split(",").map(|n| 
                                                                n.parse::<u64>().unwrap()).collect();
        items.push(monkey_items);

        let monkey_operation = lines.next().unwrap();
        operations.push(monkey_operation);

        let test_number = lines.next().unwrap().parse::<u64>().unwrap();
        test_numbers.push(test_number);

        let true_throw = lines.next().unwrap().parse::<usize>().unwrap();
        true_throws.push(true_throw);

        let false_throw = lines.next().unwrap().parse::<usize>().unwrap();
        false_throws.push(false_throw);
        
        inspects.push(0);
    }

    for _ in 0..NO_ROUNDS {
        for monkey in 0..NO_MONKEYS {
            for item in items[monkey].clone() {
                let mut value: u64 = Expr::new(operations[monkey]).value("old", item).exec().unwrap().as_u64().unwrap();
                value %= 7 * 19 * 5 * 11 * 17 * 13 * 2 * 3;

                if value % test_numbers[monkey] == 0 {
                    items[true_throws[monkey]].push(value);
                } else {
                    items[false_throws[monkey]].push(value);
                }
                inspects[monkey] += 1;

            }
            items[monkey].clear();
        }
    }
    dbg!(&inspects);
}
