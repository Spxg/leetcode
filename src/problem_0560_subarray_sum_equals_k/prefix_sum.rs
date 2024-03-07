pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut result = 0;
        let mut cache = HashMap::with_capacity(nums.len() + 1);
        cache.insert(0, 1);

        for num in nums {
            sum += num;
            if let Some(&count) = cache.get(&(sum - k)) {
                result += count;
            }
            *cache.entry(sum).or_default() += 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarray_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
