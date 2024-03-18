pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut temp = 0;
        for num in nums {
            if num == 1 {
                temp += 1;
            } else {
                result = result.max(temp);
                temp = 0;
            }
        }
        result = result.max(temp);
        result
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
