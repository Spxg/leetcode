pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }

        let mut result = 0;
        let mut left = 0;
        let mut mul = 1;

        for (idx, &num) in nums.iter().enumerate() {
            mul *= num;
            while mul >= k {
                mul /= nums[left];
                left += 1;
            }
            result += idx - left + 1;
        }

        result as i32
    }
}

impl super::Solution for Solution {
    fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        Self::num_subarray_product_less_than_k(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
