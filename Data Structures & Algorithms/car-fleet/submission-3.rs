impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut pairs = vec![];
        for i in 0..position.len() {
            pairs.push((position[i], speed[i]));
        }
        pairs.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by position in descending order
        eprintln!("Sorted pairs: {:?}", pairs);
        let mut fleets: Vec<f64> = vec![];
        for i in 0..position.len() {
            let cur_time = (target - pairs[i].0) as f64 / pairs[i].1 as f64;
            eprintln!("current last in fleets: {:?}", fleets.last());
            eprintln!("needed_time for current car: {}", cur_time);
            if fleets.is_empty() || fleets.last().unwrap() < &cur_time {
                fleets.push(cur_time);
            }
        }

        fleets.len() as i32
    }
}
