use std::fs;
fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input ="bvwbjplbgvbhsrlpgdmjqwftvnczbvwbjplbgvbhsrlpgdmjqwftvnczbvwbjplbgvbhsrlpgdmjqwftvncz";
    let count = &input.chars().count();
    'outer: for i in 13..*count {
        for x in (i-13)..=i {
            for y in (i-13)..=i {
                if x == y {
                    continue;
                }
                if &input.chars().nth(x).unwrap() == &input.chars().nth(y).unwrap() {
                    continue 'outer;
                }
            }
        }
        dbg!(i);
        break;

    }

}
