use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,

}
impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = self.map.entry(key).or_insert(Vec::new());
        entry.push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let Some(entry) = self.map.get(&key) else {
            return String::new();
        };

        let mut left = 0;
        let mut right = entry.len() as i32 - 1;
        let mut res = "".to_owned();
        while left <= right {
            let mid = (left + right) / 2;
            let cur_entry = &entry[mid as usize];
            if cur_entry.0 <= timestamp {
                res = cur_entry.1.clone();
            }

            if cur_entry.0 <= timestamp {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        res
    }
}
