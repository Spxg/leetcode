pub struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![-1; nums.len()];
        let mut stack = Vec::with_capacity(nums.len());
        for idx in 0..nums.len() * 2 {
            let idx = idx % nums.len();
            while let Some(i) = stack.pop() {
                if nums[i] < nums[idx] {
                    result[i] = nums[idx];
                } else {
                    stack.push(i);
                    break;
                }
            }
            stack.push(idx);
        }
        result
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
