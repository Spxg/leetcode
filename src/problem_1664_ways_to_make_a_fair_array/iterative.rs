pub struct Solution;

impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = (0, 0);
        let mut right = nums
            .iter()
            .enumerate()
            .fold((0, 0), |(left, right), (pos, &x)| {
                if pos % 2 == 0 {
                    (left + x, right)
                } else {
                    (left, right + x)
                }
            });

        for (idx, &val) in nums.iter().enumerate() {
            right = (right.1, right.0 - val);
            left = if idx % 2 == 0 {
                if (left.0 + right.0) == (left.1 + right.1) {
                    result += 1;
                }
                (left.0 + val, left.1)
            } else {
                if (left.0 + right.1) == (left.1 + right.0) {
                    result += 1;
                }
                (left.0, left.1 + val)
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        Self::ways_to_make_fair(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
