pub struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);
        let (mut min, mut max) = (i32::MAX, i32::MIN);
        let (mut start, mut end) = (0, -1);

        while right >= 0 {
            if nums[right as usize] <= min {
                min = nums[right as usize];
            } else {
                start = right;
            }
            if nums[left as usize] >= max {
                max = nums[left as usize];
            } else {
                end = left;
            }
            right -= 1;
            left += 1;
        }

        end - start + 1
    }
}

impl super::Solution for Solution {
    fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        Self::find_unsorted_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
