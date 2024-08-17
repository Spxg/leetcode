pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut result = 0;
        for idx in 0..strs[0].len() {
            if strs
                .windows(2)
                .any(|x| x[0].as_bytes()[idx] > x[1].as_bytes()[idx])
            {
                result += 1;
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn min_deletion_size(strs: Vec<String>) -> i32 {
        Self::min_deletion_size(strs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
