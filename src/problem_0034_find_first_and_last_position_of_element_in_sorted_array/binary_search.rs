pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start = nums.partition_point(|x| *x < target);
        let len = nums[start..].partition_point(|x| *x <= target);
        if len == 0 {
            vec![-1, -1]
        } else {
            vec![start as i32, (start + len - 1) as i32]
        }
    }
}

impl super::Solution for Solution {
    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::search_range(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
