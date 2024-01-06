pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let heap = nums.into_iter().zip(0..).collect::<BinaryHeap<_>>();
        heap.peek().unwrap().1
    }
}

impl super::Solution for Solution {
    fn find_peak_element(nums: Vec<i32>) -> i32 {
        Self::find_peak_element(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
