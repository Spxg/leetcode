pub struct Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        fn helper(nums: &[i32]) -> bool {
            if nums.len() <= 2 {
                return true;
            }
            let mut nums = nums.to_vec();
            nums.sort_unstable();
            let difference = nums[0] - nums[1];
            nums.windows(2).all(|x| x[0] - x[1] == difference)
        }

        l.into_iter()
            .zip(r)
            .map(|(l, r)| helper(&nums[l as usize..=r as usize]))
            .collect()
    }
}

impl super::Solution for Solution {
    fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        Self::check_arithmetic_subarrays(nums, l, r)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
