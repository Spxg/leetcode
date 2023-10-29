pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        fn jump_inner(nums: &[i32], step: &mut i32) {
            let target = nums.len() - 1;
            if target == 0 {
                return;
            }
            let pos = nums
                .iter()
                .enumerate()
                .position(|(idx, ele)| idx + *ele as usize >= target)
                .unwrap();
            *step += 1;
            jump_inner(&nums[0..=pos], step);
        }

        let mut count = 0;
        jump_inner(&nums, &mut count);
        count
    }
}

impl super::Solution for Solution {
    fn jump(nums: Vec<i32>) -> i32 {
        Self::jump(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
