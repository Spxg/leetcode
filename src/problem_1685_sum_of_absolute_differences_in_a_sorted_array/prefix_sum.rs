pub struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let mut prefix_sum = Vec::with_capacity(nums.len());
        let mut sum = 0;
        prefix_sum.push(sum);
        for &num in &nums[1..] {
            sum += (nums[0] - num).abs();
            prefix_sum.push(sum);
        }
        result.push(prefix_sum.last().copied().unwrap());

        for idx in 1..nums.len() {
            let offset = nums[idx] - nums[0];
            let sum = (idx as i32) * offset - prefix_sum[idx - 1]
                + (prefix_sum[nums.len() - 1] - prefix_sum[idx])
                - ((nums.len() - 1) as i32 - idx as i32) * offset;
            result.push(sum);
        }

        result
    }
}

impl super::Solution for Solution {
    fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        Self::get_sum_absolute_differences(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
