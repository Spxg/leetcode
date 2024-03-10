pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut sum = 0;
        let mut cache = HashMap::with_capacity(nums.len() + 1);
        cache.insert(0, 0);

        for (idx, num) in (1..).zip(nums) {
            sum += num;
            if let Some(&prev) = cache.get(&(sum % k)) {
                if idx - prev > 1 {
                    return true;
                }
            } else {
                cache.insert(sum % k, idx);
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        Self::check_subarray_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
