pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for num in arr {
            if set.contains(&(num * 2)) || num % 2 == 0 && set.contains(&(num / 2)) {
                return true;
            }
            set.insert(num);
        }
        false
    }
}

impl super::Solution for Solution {
    fn check_if_exist(arr: Vec<i32>) -> bool {
        Self::check_if_exist(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
