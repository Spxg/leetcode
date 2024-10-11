pub struct Solution;

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|x| (x[0], -x[1]));

        let mut end_max = 0;
        for interval in intervals {
            let end = interval[1];
            if end > end_max {
                result += 1;
            }
            end_max = end_max.max(end);
        }
        result
    }
}

impl super::Solution for Solution {
    fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        Self::remove_covered_intervals(intervals)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
