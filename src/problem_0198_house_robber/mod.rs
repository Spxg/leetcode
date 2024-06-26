pub mod dp;

pub trait Solution {
    fn rob(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 1] as &[_], 4), (&[2, 7, 9, 3, 1], 12)];

        for (nums, expected) in test_cases {
            assert_eq!(S::rob(nums.to_vec()), expected);
        }
    }
}
