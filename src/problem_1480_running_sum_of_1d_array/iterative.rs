pub struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let mut sum = 0;
        for num in nums {
            sum += num;
            result.push(sum);
        }
        result
    }
}

impl super::Solution for Solution {
    fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        Self::running_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
