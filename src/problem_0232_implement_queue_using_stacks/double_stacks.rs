pub struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            stack1: vec![],
            stack2: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.stack1.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.stack2.is_empty() {
            while let Some(v) = self.stack1.pop() {
                self.stack2.push(v);
            }
        }
        self.stack2.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        if self.stack2.is_empty() {
            self.stack1.first().copied().unwrap()
        } else {
            self.stack2.last().copied().unwrap()
        }
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }
}

impl super::MyQueue for MyQueue {
    fn new() -> Self {
        Self::new()
    }

    fn push(&mut self, x: i32) {
        self.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.pop()
    }

    fn peek(&self) -> i32 {
        self.peek()
    }

    fn empty(&self) -> bool {
        self.empty()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MyQueue>();
    }
}
