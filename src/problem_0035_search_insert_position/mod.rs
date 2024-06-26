pub mod binary_search;
pub mod binary_search_2;

pub trait Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 5, 6] as &[_], 5), 2),
            ((&[1, 3, 5, 6], 2), 1),
            ((&[1, 3, 5, 6], 7), 4),
            ((&[1, 3, 5, 6], 0), 0),
            ((&[1], 0), 0),
            ((&[1], 2), 1),
            ((&[], 4), 0),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::search_insert(nums.to_vec(), target), expected);
        }
    }
}
