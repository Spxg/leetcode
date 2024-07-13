pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let mut prev = 0;
        for (idx, val) in nums.into_iter().enumerate() {
            if sum - prev - val == prev {
                return idx as i32;
            }
            prev += val;
        }
        -1
    }
}

impl super::Solution for Solution {
    fn pivot_index(nums: Vec<i32>) -> i32 {
        Self::pivot_index(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
