pub struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len())
            .map(|i| {
                nums.iter()
                    .cycle()
                    .copied()
                    .skip(i + 1)
                    .take(nums.len() - 1)
                    .find(|&n| n > nums[i])
                    .unwrap_or(-1)
            })
            .collect()
    }
}

impl super::Solution for Solution {
    fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        Self::next_greater_elements(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
