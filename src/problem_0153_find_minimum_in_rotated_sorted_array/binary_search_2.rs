pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let first = nums[0];
        let pos = nums.partition_point(|&x| x >= first);
        nums[pos % nums.len()]
    }
}

impl super::Solution for Solution {
    fn find_min(nums: Vec<i32>) -> i32 {
        Self::find_min(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
