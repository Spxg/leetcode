pub struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|&x| nums[x as usize]).collect()
    }
}

impl super::Solution for Solution {
    fn build_array(nums: Vec<i32>) -> Vec<i32> {
        Self::build_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
