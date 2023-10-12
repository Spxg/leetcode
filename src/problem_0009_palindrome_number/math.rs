pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut num = x;
        let mut rev_num: i32 = 0;

        while num != 0 {
            rev_num = rev_num.wrapping_mul(10).wrapping_add(num % 10);
            num /= 10;
        }

        x == rev_num
    }
}

impl super::Solution for Solution {
    fn is_palindrome(x: i32) -> bool {
        Self::is_palindrome(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
