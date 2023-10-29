pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(nums: &[i32], element: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if let Some((&first, last)) = nums.split_first() {
                element.push(first);
                result.push(element.clone());
                for idx in 0..last.len() {
                    helper(&last[idx..], element, result);
                    element.pop();
                }
            }
        }

        let mut result = vec![vec![]];
        (0..nums.len()).for_each(|idx| helper(&nums[idx..], &mut Vec::new(), &mut result));

        result
    }
}

impl super::Solution for Solution {
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::subsets(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
