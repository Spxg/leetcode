pub struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        time_series
            .windows(2)
            .fold(duration, |acc, x| acc + duration.min(x[1] - x[0]))
    }
}

impl super::Solution for Solution {
    fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        Self::find_poisoned_duration(time_series, duration)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
