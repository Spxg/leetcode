pub struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        loop {
            let mid = left + (right - left) / 2;
            if mid == left {
                return nums[mid];
            }
            if nums[mid] == nums[mid - 1] {
                if (mid - left) % 2 == 0 {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else if nums[mid] == nums[mid + 1] {
                if (mid - left) % 2 == 0 {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            } else {
                return nums[mid];
            }
        }
    }
}

impl super::Solution for Solution {
    fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        Self::single_non_duplicate(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
