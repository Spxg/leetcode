pub struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::with_capacity(nums.len());
        let mut prev = 0;
        for num in nums {
            prev = (prev * 2 + i32::from(num == 1)) % 5;
            result.push(prev == 0);
        }
        result
    }
}

impl super::Solution for Solution {
    fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        Self::prefixes_div_by5(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
