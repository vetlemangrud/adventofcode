use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    part2(input);
}
fn part1(input: String) {
    let mut total = 0;
    for p in input.split("\n\n") {
        let pattern :Vec<Vec<char>> =
        p.lines().map(
            |line|
            line.chars().collect()
        ).collect();
        let height = pattern.len();
        let width = pattern[0].len();

        let mut mirror = (0,0);
        'cm: for i in 0..width-1 {
            for y in 0..height {
                for x in 0..=i {
                    if 1+2*i-x < width && pattern[y][x] != pattern[y][1+2*i-x] {
                        continue 'cm;
                    }
                }
            }

            mirror = (i+1,0);
        } 'cm: for i in 0..height-1 {
            for y in 0..=i {
                for x in 0..width {
                    if 1+2*i-y < height && pattern[y][x] != pattern[1+2*i-y][x] {
                        continue 'cm;
                    }
                }
            }
            mirror = (0,i+1);
        } 
        total += mirror.0 + 100 * mirror.1;
    }
    dbg!(total);


}
fn part2(input: String) {
    let mut total = 0;
    for p in input.split("\n\n") {
        let pattern :Vec<Vec<char>> =
        p.lines().map(
            |line|
            line.chars().collect()
        ).collect();
        let height = pattern.len();
        let width = pattern[0].len();

        let mut mirror = (0,0);
        'cm: for i in 0..width-1 {
            let mut errors = 0;
            for y in 0..height {
                for x in 0..=i {
                    if 1+2*i-x < width && pattern[y][x] != pattern[y][1+2*i-x] {
                        errors += 1;
                    }
                }
            }
           if errors == 1 {

            mirror = (i+1,0);
           } 
        } 'cm: for i in 0..height-1 {
            let mut errors = 0;
            for y in 0..=i {
                for x in 0..width {
                    if 1+2*i-y < height && pattern[y][x] != pattern[1+2*i-y][x] {
                        errors += 1;
                    }
                }
            }
            if errors == 1 {

            mirror = (0,i+1);
           } 
        } 
        total += mirror.0 + 100 * mirror.1;
    }
    dbg!(total);


}
