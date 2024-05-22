pub struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let n = i64::from(n);
        let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = left + (right - left) / 2;
            if mid * (mid + 1) <= 2 * n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32 - 1
    }
}

impl super::Solution for Solution {
    fn arrange_coins(n: i32) -> i32 {
        Self::arrange_coins(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
