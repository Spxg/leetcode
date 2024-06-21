pub struct Solution;

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut worker = worker;
        let mut jobs = difficulty.into_iter().zip(profit).collect::<Vec<_>>();
        jobs.sort_unstable();
        worker.sort_unstable();

        let (mut idx, mut max) = (0, 0);
        for ability in worker {
            while idx < jobs.len() && jobs[idx].0 <= ability {
                max = jobs[idx].1.max(max);
                idx += 1;
            }
            result += max;
        }
        result
    }
}

impl super::Solution for Solution {
    fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        Self::max_profit_assignment(difficulty, profit, worker)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
