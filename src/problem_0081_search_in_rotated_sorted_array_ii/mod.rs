pub mod binary_search;

pub trait Solution {
    fn search(nums: Vec<i32>, target: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 5, 6, 0, 0, 1, 2] as &[_], 0), true),
            ((&[2, 5, 6, 0, 0, 1, 2], 3), false),
            ((&[4, 5, 6, 7, 0, 1, 2], 7), true),
            ((&[1], 1), true),
            ((&[1, 0, 1, 1, 1], 0), true),
            ((&[1, 1, 3], 0), false),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::search(nums.to_vec(), target), expected);
        }
    }
}
