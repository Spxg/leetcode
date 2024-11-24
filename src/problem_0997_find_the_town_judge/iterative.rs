pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut map = (1..=n).zip(std::iter::repeat(0)).collect::<HashMap<_, _>>();
        for trust in trust {
            let p1 = trust[0];
            let p2 = trust[1];
            map.remove(&p1);
            if let Some(val) = map.get_mut(&p2) {
                *val += 1;
            }
        }
        let mut iter = map.into_iter().filter(|x| x.1 == n - 1);
        iter.next().map_or(-1, |x| x.0)
    }
}

impl super::Solution for Solution {
    fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        Self::find_judge(n, trust)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
