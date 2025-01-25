pub struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.repeat(2)
    }
}

impl super::Solution for Solution {
    fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        Self::get_concatenation(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
