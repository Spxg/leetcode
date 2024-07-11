struct RLEIterator {
    encoding: Vec<i32>,
    idx: usize,
}

impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        Self { encoding, idx: 0 }
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut n = n;
        loop {
            if self.idx >= self.encoding.len() {
                break -1;
            }
            let size = self.encoding[self.idx];
            if n <= size {
                self.encoding[self.idx] -= n;
                break self.encoding[self.idx + 1];
            }
            n -= size;
            self.idx += 2;
        }
    }
}

impl super::RLEIterator for RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        Self::new(encoding)
    }

    fn next(&mut self, n: i32) -> i32 {
        self.next(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::RLEIterator>();
    }
}
