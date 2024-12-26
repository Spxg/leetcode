pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut result = vec![0; k as usize];
        let mut uam: HashMap<i32, HashSet<i32>> = HashMap::new();
        for log in logs {
            uam.entry(log[0]).or_default().insert(log[1]);
        }
        for (_, u) in uam {
            result[u.len() - 1] += 1;
        }
        result
    }
}

impl super::Solution for Solution {
    fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        Self::finding_users_active_minutes(logs, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
