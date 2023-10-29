pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(nums: &[i32], element: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if let Some((&first, last)) = nums.split_first() {
                element.push(first);
                result.push(element.clone());
                let mut value = None;
                for idx in 0..last.len() {
                    if value != Some(last[idx]) {
                        helper(&last[idx..], element, result);
                        value = element.pop();
                    }
                }
            }
        }

        let mut nums = nums;
        nums.sort_unstable();

        let mut result = vec![vec![]];
        let mut prev = None;
        (0..nums.len()).for_each(|idx| {
            if prev != Some(nums[idx]) {
                helper(&nums[idx..], &mut Vec::new(), &mut result);
                prev = Some(nums[idx]);
            }
        });

        result
    }
}

impl super::Solution for Solution {
    fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::subsets_with_dup(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
