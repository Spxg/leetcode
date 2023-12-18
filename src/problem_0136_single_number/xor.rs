pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

impl super::Solution for Solution {
    fn single_number(nums: Vec<i32>) -> i32 {
        Self::single_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
