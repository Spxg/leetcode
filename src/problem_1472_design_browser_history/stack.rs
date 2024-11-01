struct BrowserHistory {
    history: Vec<String>,
    idx: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            history: vec![homepage],
            idx: 0,
        }
    }

    fn visit(&mut self, url: String) {
        if self.history.len() > self.idx + 1 {
            self.history[self.idx + 1] = url;
        } else {
            self.history.push(url);
        }
        self.idx += 1;
        self.history.truncate(self.idx + 1);
    }

    fn back(&mut self, steps: i32) -> String {
        if self.idx >= steps as usize {
            self.idx -= steps as usize;
        } else {
            self.idx = 0;
        }
        self.history[self.idx].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        if (self.idx + steps as usize) < self.history.len() {
            self.idx += steps as usize;
        } else {
            self.idx = self.history.len() - 1;
        }
        self.history[self.idx].clone()
    }
}

impl super::BrowserHistory for BrowserHistory {
    fn new(homepage: String) -> Self {
        Self::new(homepage)
    }

    fn visit(&mut self, url: String) {
        self.visit(url);
    }

    fn back(&mut self, steps: i32) -> String {
        self.back(steps)
    }

    fn forward(&mut self, steps: i32) -> String {
        self.forward(steps)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::BrowserHistory>();
    }
}
