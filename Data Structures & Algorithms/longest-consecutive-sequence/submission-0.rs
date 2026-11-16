impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut longest_streak = 0;
    let mut nums_internal = nums;
    nums_internal.sort();
    let hash_set: std::collections::HashSet<i32> = std::collections::HashSet::from_iter(nums_internal.iter().cloned());
    for num in nums_internal.iter() {
        if hash_set.contains(&(num - 1)) {
            continue;
        } 

        let mut current_num = *num;
        let mut current_streak = 1;

        while hash_set.contains(&(current_num + 1)) {
            current_num += 1;
            current_streak += 1;
        }

        longest_streak = longest_streak.max(current_streak);
    }
    longest_streak
    }
}
