pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut min: i32 = 0;
        let mut result: i32 = nums[0];

        nums.into_iter().fold(0, |acc, x| {
            min = min.min(acc);
            result = result.max(acc + x - min);
            acc + x
        });

        result
    }
}

impl super::Solution for Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        Self::max_sub_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
