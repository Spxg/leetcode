pub mod double_swap;

pub trait Solution {
    fn sort_colors(nums: &mut Vec<i32>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 0, 2, 1, 1, 0] as &[_], &[0, 0, 1, 1, 2, 2] as &[_])];

        for (nums, expected) in test_cases {
            let mut nums = nums.to_vec();

            S::sort_colors(&mut nums);

            assert_eq!(nums, expected);
        }
    }
}
