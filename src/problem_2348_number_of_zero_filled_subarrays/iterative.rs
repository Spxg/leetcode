pub struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut inc = 1;
        let mut result = 0;
        for num in nums {
            if num == 0 {
                result += inc;
                inc += 1;
            } else {
                inc = 1;
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        Self::zero_filled_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
