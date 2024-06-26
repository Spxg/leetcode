pub mod queue;

pub trait MyStack {
    fn new() -> Self;
    fn push(&mut self, x: i32);
    fn pop(&mut self) -> i32;
    fn top(&self) -> i32;
    fn empty(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::MyStack;

    enum Operation {
        Push(i32),
        Pop(i32),
        Top(i32),
        Empty(bool),
    }

    pub fn run<S: MyStack>() {
        use Operation::{Empty, Pop, Push, Top};

        let test_cases = [
            &[Push(1), Push(2), Top(2), Pop(2), Empty(false)] as &[_],
            &[
                Push(1),
                Push(2),
                Push(3),
                Pop(3),
                Pop(2),
                Pop(1),
                Empty(true),
            ] as &[_],
        ];

        for test_case in test_cases {
            let mut stack = S::new();

            for operation in test_case {
                match *operation {
                    Push(value) => stack.push(value),
                    Pop(value) => assert_eq!(stack.pop(), value),
                    Top(value) => assert_eq!(stack.top(), value),
                    Empty(value) => assert_eq!(stack.empty(), value),
                }
            }
        }
    }
}
