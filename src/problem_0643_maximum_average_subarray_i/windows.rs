pub struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let n = k as usize;
        let mut sum: i32 = nums[..n].iter().sum();
        let mut max_sum = sum;

        for i in n..nums.len() {
            sum -= nums[i - n];
            sum += nums[i];
            max_sum = max_sum.max(sum);
        }

        f64::from(max_sum) / f64::from(k)
    }
}

impl super::Solution for Solution {
    fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        Self::find_max_average(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
