pub struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if nums[i] <= 0 || nums[i] > nums.len() as i32 {
                nums[i] = i32::MAX;
            }
        }

        for i in 0..nums.len() {
            let idx = nums[i].abs() - 1;
            if idx != i32::MAX - 1 {
                nums[idx as usize] = -nums[idx as usize].abs();
            }
        }

        for (i, &v) in nums.iter().enumerate() {
            if v > 0 {
                return i as i32 + 1;
            }
        }

        nums.len() as i32 + 1
    }
}

impl super::Solution for Solution {
    fn first_missing_positive(nums: Vec<i32>) -> i32 {
        Self::first_missing_positive(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
