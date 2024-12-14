pub struct Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        fn helper(n: i32) -> bool {
            let mut n = n;
            while n != 0 {
                if n % 10 == 0 {
                    return false;
                }
                n /= 10;
            }
            true
        }
        for x in 1..=n / 2 {
            if helper(x) && helper(n - x) {
                return vec![x, n - x];
            }
        }
        unreachable!()
    }
}

impl super::Solution for Solution {
    fn get_no_zero_integers(n: i32) -> Vec<i32> {
        Self::get_no_zero_integers(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
