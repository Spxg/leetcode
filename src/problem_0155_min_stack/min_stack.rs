struct MinStack {
    inner: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn push(&mut self, val: i32) {
        let min = self.inner.last().map_or(val, |&(_, min)| min.min(val));
        self.inner.push((val, min));
    }

    fn pop(&mut self) {
        self.inner.pop().unwrap();
    }

    fn top(&self) -> i32 {
        self.inner.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.inner.last().unwrap().1
    }
}

impl super::MinStack for MinStack {
    fn new() -> Self {
        Self::new()
    }

    fn push(&mut self, x: i32) {
        self.push(x);
    }

    fn pop(&mut self) {
        self.pop();
    }

    fn top(&self) -> i32 {
        self.top()
    }

    fn get_min(&self) -> i32 {
        self.get_min()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MinStack>();
    }
}
