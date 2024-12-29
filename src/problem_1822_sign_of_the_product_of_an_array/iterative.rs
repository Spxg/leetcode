pub struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(i32::signum).product::<i32>().signum()
    }
}

impl super::Solution for Solution {
    fn array_sign(nums: Vec<i32>) -> i32 {
        Self::array_sign(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
