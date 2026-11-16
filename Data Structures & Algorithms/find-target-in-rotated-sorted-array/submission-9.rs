impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if target == nums[mid] {
            return mid as i32;
        }

        let is_left_sorted = nums[mid] > nums[right];
        if is_left_sorted {
            if target >= nums[left] && target <= nums[mid] {
                if mid == 0 {
                    break; // Prevent underflow
                }
                right = mid - 1;
            } else {
                if mid >= nums.len() - 1 {
                    break; // Prevent overflow
                }
                left = mid + 1;
            }
        } else {
            if target >= nums[mid] && target <= nums[right] {
                if mid >= nums.len() - 1 {
                    break; // Prevent overflow
                }
                left = mid + 1;
            } else {
                if mid == 0 {
                    break; // Prevent underflow
                }
                right = mid - 1;
            }
        }
    }
    -1
    }
}
