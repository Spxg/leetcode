pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for num in nums {
            *map.entry(num).or_default() += 1;
        }
        map.iter()
            .filter_map(|(key, val)| map.get(&(key + 1)).map(|&x| val + x))
            .max()
            .unwrap_or(0)
    }
}

impl super::Solution for Solution {
    fn find_lhs(nums: Vec<i32>) -> i32 {
        Self::find_lhs(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
