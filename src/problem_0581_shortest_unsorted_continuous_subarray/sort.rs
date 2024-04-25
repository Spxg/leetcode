pub struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let nums1 = nums.clone();
        let mut nums = nums;
        nums.sort_unstable();
        let mut iter = nums
            .iter()
            .zip(&nums1)
            .enumerate()
            .filter_map(|(pos, (x, y))| (x != y).then_some(pos));
        let left = iter.next();
        let right = iter.next_back();
        match (left, right) {
            (None, None) => 0,
            (Some(x), Some(y)) => (y - x + 1) as i32,
            _ => unreachable!(),
        }
    }
}

impl super::Solution for Solution {
    fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        Self::find_unsorted_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
