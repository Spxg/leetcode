use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let set: BTreeSet<_> = nums.iter().copied().collect();
        for (idx, v) in set.iter().enumerate() {
            nums[idx] = *v;
        }
        set.len() as i32
    }
}

impl super::Solution for Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        Self::remove_duplicates(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
