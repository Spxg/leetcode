pub struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums[nums.len() - 3..]
            .iter()
            .product::<i32>()
            .max(nums[0] * nums[1] * nums[nums.len() - 1])
    }
}

impl super::Solution for Solution {
    fn maximum_product(nums: Vec<i32>) -> i32 {
        Self::maximum_product(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
