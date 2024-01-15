pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut result = 0;
        for (idx, val) in (1..).zip(citations.into_iter().rev()) {
            if idx > val {
                break;
            }
            result = idx;
        }
        result
    }
}

impl super::Solution for Solution {
    fn h_index(citations: Vec<i32>) -> i32 {
        Self::h_index(citations)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
