pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        (len * (len + 1) / 2) - nums.into_iter().sum::<i32>()
    }
}

impl super::Solution for Solution {
    fn missing_number(nums: Vec<i32>) -> i32 {
        Self::missing_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
