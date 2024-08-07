pub mod iterative;

pub trait Solution {
    fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 4] as &[_], 2), 4), ((&[1, 2], 2), 3)];

        for ((time_series, duration), expected) in test_cases {
            assert_eq!(
                S::find_poisoned_duration(time_series.to_vec(), duration),
                expected
            );
        }
    }
}
