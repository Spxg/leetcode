pub struct Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        fn helper(nums: &[i32], mid: i32) -> (i32, i32) {
            nums.iter().fold((0, 0), |acc, &val| {
                if val < mid {
                    (acc.0 + 1, acc.1)
                } else {
                    (acc.0, acc.1 + 1)
                }
            })
        }

        let mut result = 0;
        for (idx, &mid) in rating.iter().enumerate() {
            let left = helper(&rating[0..idx], mid);
            let right = helper(&rating[idx + 1..], mid);
            result += left.0 * right.1 + left.1 * right.0;
        }
        result
    }
}

impl super::Solution for Solution {
    fn num_teams(rating: Vec<i32>) -> i32 {
        Self::num_teams(rating)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
