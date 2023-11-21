use std::fs;
use std::cmp::max;

fn max_geodes(time_left: u32, resources: &Vec<u32>, to_activate: &Vec<u32>, robots: &Vec<u32>, prices: &Vec<Vec<u32>>, buy_streak: u32) -> u32 {
    if time_left == 0 {
        return resources[3];
    }

    let wait_score = {
        let mut new_res = resources.clone();
        let mut new_robots = robots.clone();
        let new_buy_streak = if to_activate.iter().all(|r| *r == 0) {0} else {buy_streak};
        for i in 0..4 {
            new_res[i] += robots[i];
            new_robots[i] += to_activate[i];
        }
        max_geodes(time_left-1, &new_res, &vec![0,0,0,0], &new_robots, prices, new_buy_streak)
    };
    if to_activate.iter().any(|r| *r == 1) {return wait_score;}

    let buy_score = {
        let mut scores = vec![0];
        for i in (0..4).rev() {
            let mut enough = true;
            let mut barely = false;
            for j in 0..4 {
                if prices[i][j] > resources[j] - robots[j] {
                    barely = true;
                }
                if prices[i][j] > resources[j] {
                    enough = false;
                }
            }
            if enough && (barely || buy_streak > 0) {
                let mut new_res = resources.clone();
                let mut new_to_activate = to_activate.clone();
                for j in 0..4 {
                    new_res[j] -= prices[i][j];
                }
                new_to_activate[i] += 1;
                scores.push(max_geodes(time_left, &new_res, &new_to_activate ,robots, prices, buy_streak + 1));
                if i == 3 {break;}
            }
        }
        *scores.iter().max().unwrap()
    };

    max(wait_score, buy_score)
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    //let input = "2,2,3,3,8,3,12";
    let mut quality_sum = 1;
    for line in input.lines().take(3){
        dbg!(line);
        let args: Vec<u32> = line.split(",").map(|n| n.parse::<u32>().unwrap()).collect();
        let resources = vec![0,0,0,0];
        let to_activate = vec![0,0,0,0];
        let robots = vec![1,0,0,0];
        let prices = vec![
            vec![args[1],0,0,0],
            vec![args[2],0,0,0],
            vec![args[3],args[4],0,0],
            vec![args[5],0,args[6],0]
        ];
        quality_sum *=  max_geodes(32, &resources, &to_activate, &robots, &prices, 0);
        dbg!(&quality_sum);
    }
    dbg!(quality_sum);
}
