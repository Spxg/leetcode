use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct AuthenticationManager {
    time_to_live: i32,
    map: HashMap<String, i32>,
    heap: BinaryHeap<Reverse<(i32, String)>>,
}

impl AuthenticationManager {
    #[allow(non_snake_case)]
    fn new(timeToLive: i32) -> Self {
        Self {
            time_to_live: timeToLive,
            map: HashMap::new(),
            heap: BinaryHeap::new(),
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        let expired = current_time + self.time_to_live;
        self.map.insert(token_id.clone(), expired);
        self.heap.push(Reverse((expired, token_id)));
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        if let Some(expired) = self.map.remove(&token_id) {
            if current_time < expired {
                self.generate(token_id, current_time);
            }
        }
    }

    fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        while let Some(Reverse((expired, token))) = self.heap.peek().cloned() {
            if expired <= current_time {
                if let Some(e) = self.map.get(&token).copied() {
                    if e == expired {
                        self.map.remove(&token);
                    }
                }
                self.heap.pop();
            } else {
                break;
            }
        }
        self.map.len() as _
    }
}

impl super::AuthenticationManager for AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        Self::new(time_to_live)
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.generate(token_id, current_time);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        self.renew(token_id, current_time);
    }

    fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        self.count_unexpired_tokens(current_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::AuthenticationManager>();
    }
}
