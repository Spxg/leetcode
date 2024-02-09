pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut [i32], k: i32) {
        let len = nums.len();
        nums.rotate_right(k as usize % len);
    }
}

impl super::Solution for Solution {
    fn rotate(nums: &mut Vec<i32>, k: i32) {
        Self::rotate(nums, k);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
