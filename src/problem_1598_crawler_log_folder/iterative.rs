pub struct Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut step = 0;
        for log in logs {
            if log.starts_with("..") {
                if step != 0 {
                    step -= 1;
                }
            } else if !log.starts_with('.') {
                step += 1;
            }
        }
        step
    }
}

impl super::Solution for Solution {
    fn min_operations(logs: Vec<String>) -> i32 {
        Self::min_operations(logs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
