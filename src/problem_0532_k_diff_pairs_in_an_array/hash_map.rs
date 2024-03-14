pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut map = HashMap::with_capacity(nums.len());
        nums.into_iter()
            .for_each(|x| *map.entry(x).or_insert(0) += 1);
        for (&x, &n) in &map {
            if map.contains_key(&(x + k)) {
                result += i32::from(k > 0 || k == 0 && n > 1);
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        Self::find_pairs(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
