pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut target = nums.len() - 1;
        for idx in (0..nums.len()).rev() {
            if nums[idx] as usize + idx >= target {
                target = idx;
            }
        }

        target == 0
    }
}

impl super::Solution for Solution {
    fn can_jump(nums: Vec<i32>) -> bool {
        Self::can_jump(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
