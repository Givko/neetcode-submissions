impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        let mut stack = Vec::<i32>::with_capacity(temperatures.len());
        for i in 0..temperatures.len() {
            while !stack.is_empty() && temperatures[i] > temperatures[*stack.last().unwrap() as usize]{
                let popped = stack.pop().unwrap();
                result[popped as usize] = i as i32 - popped; 
            }

            stack.push(i as i32); 
        }

        return result;
    }
}
