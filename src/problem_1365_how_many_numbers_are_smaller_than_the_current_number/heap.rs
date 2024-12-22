pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut map = HashMap::with_capacity(nums.len());
        let mut heap: BinaryHeap<Reverse<_>> =
            nums.into_iter().zip(0usize..).map(Reverse).collect();

        let mut count = 0;
        while let Some(Reverse((val, idx))) = heap.pop() {
            if let Some(x) = map.insert(val, idx) {
                result[idx] = result[x];
            } else {
                result[idx] = count;
            }
            count += 1;
        }
        result
    }
}

impl super::Solution for Solution {
    fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        Self::smaller_numbers_than_current(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
