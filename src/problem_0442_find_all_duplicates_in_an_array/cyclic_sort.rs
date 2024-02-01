pub struct Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for idx1 in 0..nums.len() {
            let mut num = nums[idx1];
            let expected = idx1 as i32 + 1;
            while num != expected {
                let idx2 = num as usize - 1;
                let target = nums[idx2];
                if target == num {
                    break;
                }
                nums.swap(idx1, idx2);
                num = target;
            }
        }
        nums.into_iter()
            .enumerate()
            .filter_map(|(idx, value)| (idx + 1 != value as usize).then_some(value))
            .collect()
    }
}

impl super::Solution for Solution {
    fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        Self::find_duplicates(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
