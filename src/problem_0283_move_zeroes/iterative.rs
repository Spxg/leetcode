pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero = 0;
        let mut no_zero = 1;
        while no_zero < nums.len() {
            if nums[zero] == 0 && nums[no_zero] != 0 {
                nums.swap(zero, no_zero);
                zero += 1;
            } else if nums[zero] != 0 {
                zero += 1;
            }
            no_zero += 1;
        }
    }
}

impl super::Solution for Solution {
    fn move_zeroes(nums: &mut Vec<i32>) {
        Self::move_zeroes(nums);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
