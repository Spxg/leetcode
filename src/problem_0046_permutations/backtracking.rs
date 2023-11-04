pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, cur: usize) {
            if cur == nums.len() {
                result.push(nums.clone());
            }

            for idx in cur..nums.len() {
                nums.swap(idx, cur);
                helper(nums, result, cur + 1);
                nums.swap(idx, cur);
            }
        }

        let mut nums = nums;
        let mut result = vec![];
        helper(&mut nums, &mut result, 0);
        result
    }
}

impl super::Solution for Solution {
    fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::permute(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
