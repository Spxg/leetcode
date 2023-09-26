pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        let idx = intervals
            .binary_search_by_key(&new_interval[0], |x| x[0])
            .unwrap_or_else(|x| x);

        let new_interval = [new_interval];
        let mut chain = intervals[0..idx]
            .iter()
            .chain(new_interval.iter())
            .chain(intervals[idx..].iter());

        let filst = chain.next().unwrap();
        let mut start = filst[0];
        let mut end = filst[1];

        for array in chain {
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
    fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        Self::insert(intervals, new_interval)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
