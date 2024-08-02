use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.map.get(&key).map_or_else(String::new, |value| {
            let idx = value.partition_point(|&(time, _)| time <= timestamp);
            if idx == 0 {
                String::new()
            } else {
                value[idx - 1].1.clone()
            }
        })
    }
}

impl super::TimeMap for TimeMap {
    fn new() -> Self {
        Self::new()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.set(key, value, timestamp);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.get(key, timestamp)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::TimeMap>();
    }
}
