pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in (0..).zip(nums.iter()) {
            if let Some(&index) = map.get(&(target - num)) {
                return vec![index, i];
            }
            map.insert(num, i);
        }

        // Description:
        // You may assume that each input would have exactly one solution
        unreachable!();
    }
}

impl super::Solution for Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
