pub struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut idx = nums.len() - 1;
        let mut pointer1 = 0;
        let mut pointer2 = nums.len() - 1;

        while pointer1 < pointer2 {
            let p1 = nums[pointer1] * nums[pointer1];
            let p2 = nums[pointer2] * nums[pointer2];
            result[idx] = p1.max(p2);
            if p1 < p2 {
                pointer2 -= 1;
            } else {
                pointer1 += 1;
            }
            idx -= 1;
        }

        result[idx] = nums[pointer1] * nums[pointer1];

        result
    }
}

impl super::Solution for Solution {
    fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        Self::sorted_squares(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
