pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm

        let (mut result, mut vote) = (nums[0], 1);

        for num in nums.into_iter().skip(1) {
            if num == result {
                vote += 1;
            } else if vote == 0 {
                result = num;
                vote = 1;
            } else {
                vote -= 1;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn majority_element(nums: Vec<i32>) -> i32 {
        Self::majority_element(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
