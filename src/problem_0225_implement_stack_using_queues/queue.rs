pub struct Solution;

use std::collections::VecDeque;

struct MyStack {
    quque: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            quque: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.quque.push_back(x);
        for _ in 1..self.quque.len() {
            let back = self.quque.pop_front().unwrap();
            self.quque.push_back(back);
        }
    }

    fn pop(&mut self) -> i32 {
        self.quque.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.quque.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.quque.is_empty()
    }
}

impl super::MyStack for MyStack {
    fn new() -> Self {
        Self::new()
    }

    fn push(&mut self, x: i32) {
        self.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.pop()
    }

    fn top(&self) -> i32 {
        self.top()
    }

    fn empty(&self) -> bool {
        self.empty()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MyStack>();
    }
}
