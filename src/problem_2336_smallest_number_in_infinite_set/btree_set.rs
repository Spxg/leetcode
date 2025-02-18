use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    value: i32,
    remain: BTreeSet<i32>,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            value: 1,
            remain: BTreeSet::new(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(val) = self.remain.pop_first() {
            return val;
        }
        let val = self.value;
        self.value += 1;
        val
    }

    fn add_back(&mut self, num: i32) {
        if num < self.value {
            self.remain.insert(num);
        }
    }
}

impl super::SmallestInfiniteSet for SmallestInfiniteSet {
    fn new() -> Self {
        Self::new()
    }

    fn pop_smallest(&mut self) -> i32 {
        self.pop_smallest()
    }

    fn add_back(&mut self, num: i32) {
        self.add_back(num);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::SmallestInfiniteSet>();
    }
}
