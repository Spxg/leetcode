pub struct Solution;

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut n = n;
        let mut rev = false;
        let mut offset = 1;
        let mut pow = 0;

        while n / 2 != 0 {
            if !rev || n % 2 != 0 {
                offset += 2i32.pow(pow);
            }
            rev = !rev;
            pow += 1;
            n /= 2;
        }
        offset
    }
}

impl super::Solution for Solution {
    fn last_remaining(n: i32) -> i32 {
        Self::last_remaining(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
