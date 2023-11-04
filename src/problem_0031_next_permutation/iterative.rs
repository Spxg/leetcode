pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for idx1 in (0..nums.len()).rev() {
            let slice = &mut nums[idx1..];
            for last in (0..slice.len()).rev() {
                for idx2 in 0..last {
                    if slice[idx2] < slice[last] {
                        slice.swap(idx2, last);
                        slice[idx2 + 1..].reverse();
                        return;
                    }
                }
            }
        }
        nums.reverse();
    }
}

impl super::Solution for Solution {
    fn next_permutation(nums: &mut Vec<i32>) {
        Self::next_permutation(nums);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
