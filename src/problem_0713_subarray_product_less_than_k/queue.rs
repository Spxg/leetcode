pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut dup = 0;
        let mut mul = 1;
        let mut queue = VecDeque::with_capacity(nums.len());
        for num in nums {
            loop {
                if num * mul < k {
                    queue.push_back(num);
                    mul *= num;
                    break;
                } else if let Some(first) = queue.pop_front() {
                    mul /= first;
                    let size = queue.len() as i32 + 1;
                    result += (size * (size + 1)) / 2 - (dup * (dup + 1)) / 2;
                    dup = size - 1;
                } else {
                    break;
                }
            }
        }
        let size = queue.len() as i32;
        result += (size * (size + 1)) / 2 - (dup * (dup + 1)) / 2;
        result
    }
}

impl super::Solution for Solution {
    fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        Self::num_subarray_product_less_than_k(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
