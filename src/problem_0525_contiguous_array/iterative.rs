pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::with_capacity(nums.len());
        map.insert(0, 0);

        let mut result = 0;
        let mut count = [0, 0];

        for (num, idx) in nums.into_iter().zip(1..) {
            count[num as usize] += 1;
            let key = count[1] - count[0];
            if let Some(&prev) = map.get(&key) {
                result = result.max(idx - prev);
            } else {
                map.insert(key, idx);
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_max_length(nums: Vec<i32>) -> i32 {
        Self::find_max_length(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
