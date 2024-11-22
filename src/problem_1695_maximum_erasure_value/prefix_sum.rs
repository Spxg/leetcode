pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut total_sum = 0;
        let mut left_sum = 0;
        let mut map = HashMap::new();
        for num in nums {
            total_sum += num;
            if let Some(prev) = map.insert(num, total_sum) {
                left_sum = left_sum.max(prev);
            }
            result = result.max(total_sum - left_sum);
        }

        result
    }
}

impl super::Solution for Solution {
    fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        Self::maximum_unique_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
