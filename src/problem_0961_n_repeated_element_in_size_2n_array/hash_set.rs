pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        nums.into_iter().find(|&x| !set.insert(x)).unwrap()
    }
}

impl super::Solution for Solution {
    fn repeated_n_times(nums: Vec<i32>) -> i32 {
        Self::repeated_n_times(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
