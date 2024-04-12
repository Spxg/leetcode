pub struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.windows(2)
            .map(|nums| nums[1] - nums[0])
            .max()
            .unwrap_or(0)
    }
}

impl super::Solution for Solution {
    fn maximum_gap(nums: Vec<i32>) -> i32 {
        Self::maximum_gap(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
