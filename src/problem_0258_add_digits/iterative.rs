pub struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        let mut record = 0;

        loop {
            while num != 0 {
                record += num % 10;
                num /= 10;
            }
            if record < 10 {
                return record;
            }
            num = record;
            record = 0;
        }
    }
}

impl super::Solution for Solution {
    fn add_digits(num: i32) -> i32 {
        Self::add_digits(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
