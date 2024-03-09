pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut result = 0;
        let mut queue = VecDeque::with_capacity(nums.len() + 1);
        queue.push_back((0, 0));

        for (idx1, num) in (1..).zip(nums) {
            sum += num;
            while let Some((first, idx2)) = queue.front().copied() {
                if sum - first >= target {
                    result = if result == 0 {
                        idx1 - idx2
                    } else {
                        result.min(idx1 - idx2)
                    }
                } else {
                    break;
                }
                queue.pop_front();
            }
            queue.push_back((sum, idx1));
        }

        result
    }
}

impl super::Solution for Solution {
    fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        Self::min_sub_array_len(s, nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
