pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut left = 0;
        let mut result = 0;

        for right in 0..nums.len() {
            sum += nums[right];
            while sum >= target {
                result = if result == 0 {
                    right - left + 1
                } else {
                    result.min(right - left + 1)
                };
                sum -= nums[left];
                left += 1;
            }
        }

        result as i32
    }
}

impl super::Solution for Solution {
    fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        Self::min_sub_array_len(s, nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
