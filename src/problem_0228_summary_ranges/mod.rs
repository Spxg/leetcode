pub mod iterative;

pub trait Solution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 1, 2, 4, 5, 7] as &[_], &["0->2", "4->5", "7"] as &[_]),
            (&[0, 2, 3, 4, 6, 8, 9], &["0", "2->4", "6", "8->9"]),
            (&[], &[]),
            (&[-1], &["-1"]),
            (&[0], &["0"]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::summary_ranges(nums.to_vec()), expected);
        }
    }
}
