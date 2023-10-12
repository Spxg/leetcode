pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut rev_num: i32 = 0;

        while num != 0 {
            if let Some(result) = rev_num.checked_mul(10).and_then(|x| x.checked_add(num % 10)) {
                rev_num = result;
                num /= 10;
            } else {
                return 0;
            }
        }

        rev_num
    }
}

impl super::Solution for Solution {
    fn reverse(x: i32) -> i32 {
        Self::reverse(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
