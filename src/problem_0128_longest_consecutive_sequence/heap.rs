pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut max = 0;
        let mut result = 0;
        let mut heap = nums.into_iter().collect::<BinaryHeap<_>>();

        while let Some(val) = heap.pop() {
            if max == 0 || prev - val == 1 {
                max += 1;
            } else if prev == val {
                continue;
            } else {
                max = 1;
            }
            result = result.max(max);
            prev = val;
        }

        result
    }
}

impl super::Solution for Solution {
    fn longest_consecutive(nums: Vec<i32>) -> i32 {
        Self::longest_consecutive(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
