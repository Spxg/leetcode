pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut dp = vec![(nums[0], nums[0]); nums.len()];
        for (idx, &num) in (1..).zip(nums.iter().skip(1)) {
            let (max, min) = dp[idx - 1];
            dp[idx] = match num.cmp(&0) {
                std::cmp::Ordering::Less => ((min * num).max(num), (max * num).min(num)),
                std::cmp::Ordering::Equal => (0, 0),
                std::cmp::Ordering::Greater => ((max * num).max(num), (min * num).min(num)),
            };
            result = result.max(dp[idx].0);
        }
        result
    }
}

impl super::Solution for Solution {
    fn max_product(nums: Vec<i32>) -> i32 {
        Self::max_product(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
