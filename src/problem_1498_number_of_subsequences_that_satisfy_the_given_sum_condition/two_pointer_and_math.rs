pub struct Solution;

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let modulo = 1_000_000_007;

        let mut result = 0;
        let mut nums = nums;
        nums.sort_unstable();

        let mut pow2 = vec![1; nums.len()];
        for i in 1..nums.len() {
            pow2[i] = (pow2[i - 1] * 2) % modulo;
        }

        let (mut left, mut right) = (0, nums.len() - 1);
        while left <= right {
            if nums[left] + nums[right] <= target {
                result = (result + pow2[right - left]) % modulo;
                left += 1;
            } else {
                if right == 0 {
                    break;
                }
                right -= 1;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        Self::num_subseq(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
