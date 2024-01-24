pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1 = nums1.into_iter().collect::<HashSet<_>>();
        let set2 = nums2.into_iter().collect::<HashSet<_>>();
        set1.intersection(&set2).copied().collect()
    }
}

impl super::Solution for Solution {
    fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        Self::intersection(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
