pub struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut pop_idx = 0;
        let mut stack = Vec::with_capacity(pushed.len());
        for val in pushed {
            stack.push(val);
            while let Some(v) = stack.last().copied() {
                if popped[pop_idx] == v {
                    pop_idx += 1;
                    stack.pop();
                } else {
                    break;
                }
            }
        }
        stack.is_empty()
    }
}

impl super::Solution for Solution {
    fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        Self::validate_stack_sequences(pushed, popped)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
