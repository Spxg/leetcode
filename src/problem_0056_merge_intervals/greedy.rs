pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|x| x[0]);

        let mut result = vec![];
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];

        for array in intervals.iter().skip(1) {
            if array[0] > end {
                result.push(vec![start, end]);
                start = array[0];
            }
            end = array[1].max(end);
        }

        result.push(vec![start, end]);

        result
    }
}

impl super::Solution for Solution {
    fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::merge(intervals)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
