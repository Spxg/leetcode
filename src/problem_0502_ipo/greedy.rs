pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut result = w;
        let mut projects = profits.into_iter().zip(capital).collect::<Vec<_>>();
        let mut queue = BinaryHeap::with_capacity(projects.len());

        projects.sort_unstable_by_key(|&(_, capital)| capital);

        let mut project_iter = projects.into_iter().peekable();

        for _ in 0..k {
            while let Some(profit) = project_iter
                .peek()
                .and_then(|&(profit, capital)| (capital <= result).then_some(profit))
            {
                queue.push(profit);
                project_iter.next();
            }

            if let Some(profit) = queue.pop() {
                result += profit;
            } else {
                break;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        Self::find_maximized_capital(k, w, profits, capital)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
