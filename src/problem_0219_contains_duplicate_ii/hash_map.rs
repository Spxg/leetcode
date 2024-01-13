pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::with_capacity(nums.len());
        nums.into_iter().enumerate().any(|(idx, val)| {
            map.insert(val, idx)
                .map_or(false, |x| (idx - x) <= k as usize)
        })
    }
}

impl super::Solution for Solution {
    fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        Self::contains_nearby_duplicate(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
