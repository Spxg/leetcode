use std::collections::{BinaryHeap, HashMap};

struct FreqStack {
    count: HashMap<i32, i32>,
    heap: BinaryHeap<(i32, usize, i32)>,
    priority: usize,
}

impl FreqStack {
    fn new() -> Self {
        Self {
            count: HashMap::new(),
            heap: BinaryHeap::new(),
            priority: 0,
        }
    }

    fn push(&mut self, val: i32) {
        *self.count.entry(val).or_default() += 1;
        self.heap.push((self.count[&val], self.priority, val));
        self.priority += 1;
    }

    fn pop(&mut self) -> i32 {
        let (_, _, val) = self.heap.pop().unwrap();
        *self.count.get_mut(&val).unwrap() -= 1;
        val
    }
}

impl super::FreqStack for FreqStack {
    fn new() -> Self {
        Self::new()
    }

    fn push(&mut self, val: i32) {
        self.push(val);
    }

    fn pop(&mut self) -> i32 {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::FreqStack>();
    }
}
