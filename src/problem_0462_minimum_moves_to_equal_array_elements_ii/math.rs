pub struct Solution;

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = 0;
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            result += nums[right] - nums[left];
            left += 1;
            right -= 1;
        }
        result
    }
}

impl super::Solution for Solution {
    fn min_moves2(nums: Vec<i32>) -> i32 {
        Self::min_moves2(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
