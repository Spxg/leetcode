pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        if nums[left] < nums[right] {
            return nums[0];
        }

        while right > left + 1 {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[right] {
                left = mid;
            } else {
                right = mid;
            }
        }

        nums[right]
    }
}

impl super::Solution for Solution {
    fn find_min(nums: Vec<i32>) -> i32 {
        Self::find_min(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
