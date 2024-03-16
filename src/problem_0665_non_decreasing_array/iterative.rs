pub struct Solution;

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut idx = 0;
        let mut nums = nums;
        let mut replaced = false;
        let mut min = i32::MIN;

        while idx + 1 < nums.len() {
            if nums[idx] > nums[idx + 1] {
                if replaced {
                    return false;
                }
                if nums[idx + 1] < min {
                    nums[idx + 1] = nums[idx];
                } else {
                    nums[idx] = nums[idx + 1];
                }
                replaced = true;
            }
            min = nums[idx];
            idx += 1;
        }

        true
    }
}

impl super::Solution for Solution {
    fn check_possibility(nums: Vec<i32>) -> bool {
        Self::check_possibility(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
