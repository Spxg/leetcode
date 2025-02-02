pub struct Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable_by_key(|&x| x % 2);
        nums
    }
}

impl super::Solution for Solution {
    fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_array_by_parity(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
