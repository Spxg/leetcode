struct CustomStack {
    max_size: usize,
    inner: Vec<i32>,
}

impl CustomStack {
    #[allow(non_snake_case)]
    fn new(maxSize: i32) -> Self {
        let max_size = maxSize as usize;
        Self {
            max_size,
            inner: Vec::with_capacity(max_size),
        }
    }

    fn push(&mut self, x: i32) {
        if self.inner.len() != self.max_size {
            self.inner.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.inner.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        let end = (k as usize).min(self.inner.len());
        self.inner[0..end].iter_mut().for_each(|x| *x += val);
    }
}

impl super::CustomStack for CustomStack {
    fn new(max_size: i32) -> Self {
        Self::new(max_size)
    }

    fn push(&mut self, x: i32) {
        self.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.pop()
    }

    fn increment(&mut self, k: i32, val: i32) {
        self.increment(k, val);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::CustomStack>();
    }
}
