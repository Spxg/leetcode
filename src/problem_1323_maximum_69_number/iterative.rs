pub struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut result = 0;
        let mut div = if num < 100 {
            10
        } else if num < 1000 {
            100
        } else {
            1000
        };
        let mut num = num;
        let mut ok = false;
        while num != 0 {
            let mut digit = num / div;
            num %= div;
            div /= 10;
            if digit == 6 && !ok {
                ok = true;
                digit = 9;
            }
            result = result * 10 + digit;
        }
        result
    }
}

impl super::Solution for Solution {
    fn maximum69_number(num: i32) -> i32 {
        Self::maximum69_number(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
