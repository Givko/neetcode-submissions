impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
            let mut L = 0;
            let mut R = heights.len() - 1;
            while L < R {
                let l_height = heights.get(L).unwrap();
                let r_height = heights.get(R).unwrap();
                let min_height = l_height.min(r_height);

                let area = *min_height * (R as i32 - L as i32);
                max_area = max_area.max(area);

                if l_height < r_height {
                    L += 1;
                } else if l_height > r_height {
                    R -= 1;
                } else {
                    L += 1;
                    R -= 1;
                }
            }

            return max_area;
    }
}
