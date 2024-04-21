pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut t = 0;
        let mut set = HashSet::new();
        set.insert(n);

        loop {
            while n != 0 {
                let x = n % 10;
                n /= 10;
                t += x * x;
            }
            n = t;
            t = 0;
            if n == 1 {
                return true;
            }
            if set.contains(&n) {
                return false;
            }
            set.insert(n);
        }
    }
}

impl super::Solution for Solution {
    fn is_happy(n: i32) -> bool {
        Self::is_happy(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
