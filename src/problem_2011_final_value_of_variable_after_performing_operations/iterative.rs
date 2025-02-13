pub struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.into_iter().fold(0, |acc, x| {
            acc + if x.starts_with('+') || x.ends_with('+') {
                1
            } else {
                -1
            }
        })
    }
}

impl super::Solution for Solution {
    fn final_value_after_operations(operations: Vec<String>) -> i32 {
        Self::final_value_after_operations(operations)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
