pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = 0;
        let mut min_diff = i32::MAX;

        for idx in 0..nums.len() {
            let mut pointer1 = idx + 1;
            let mut pointer2 = nums.len() - 1;

            while pointer1 < pointer2 {
                let sum = nums[idx] + nums[pointer1] + nums[pointer2];
                match sum.cmp(&target) {
                    std::cmp::Ordering::Less => pointer1 += 1,
                    std::cmp::Ordering::Equal => return target,
                    std::cmp::Ordering::Greater => pointer2 -= 1,
                }

                let diff = (sum - target).abs();
                if min_diff > diff {
                    result = sum;
                    min_diff = diff;
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        Self::three_sum_closest(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
