pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        let mut map = HashMap::with_capacity(nums.len() + 1);
        map.insert(0, 1);

        let mut prefix_sum = 0;
        for num in nums.into_iter().map(|num| i32::from(num % 2 != 0)) {
            prefix_sum += num;
            *map.entry(prefix_sum).or_insert(0) += 1;
            result += map.get(&(prefix_sum - k)).copied().unwrap_or(0);
        }
        result
    }
}

impl super::Solution for Solution {
    fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        Self::number_of_subarrays(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
