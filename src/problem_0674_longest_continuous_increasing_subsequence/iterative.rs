pub struct Solution;

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut idx = 0;
        let mut temp = 1;

        while idx + 1 < nums.len() {
            if nums[idx] < nums[idx + 1] {
                temp += 1;
            } else {
                result = result.max(temp);
                temp = 1;
            }
            idx += 1;
        }
        result = result.max(temp);
        result
    }
}

impl super::Solution for Solution {
    fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        Self::find_length_of_lcis(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
