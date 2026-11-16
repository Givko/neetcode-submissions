impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut r = *piles.iter().max().unwrap();
        let mut l = 1;
        let mut res = r;
        while l <= r {
            let k:i32 = (l + r)/2;
            let mut total_hours: i64 = 0;
            for &p in &piles {
                total_hours += (p as i64 + k as i64- 1) / k as i64;
            }

            if total_hours <= h as i64{
                res = k;
                r = k - 1;
            } else {
                l = k + 1;
            }
        }

        res
    }
}
