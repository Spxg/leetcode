pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.split(|&x| x == 0).map(<[_]>::len).max().unwrap_or(0) as _
    }
}

impl super::Solution for Solution {
    fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        Self::find_max_consecutive_ones(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
