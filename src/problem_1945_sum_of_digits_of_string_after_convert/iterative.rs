pub struct Solution;

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut total = 0;
        for num in s.bytes() {
            total += if num.is_ascii_digit() {
                num as i32
            } else {
                let num = num - b'a' + 1;
                ((num / 10) + (num % 10)) as i32
            };
        }

        for _ in 0..k - 1 {
            let mut num = total;
            total = 0;
            while num != 0 {
                total += num % 10;
                num /= 10;
            }
        }
        total
    }
}

impl super::Solution for Solution {
    fn get_lucky(s: String, k: i32) -> i32 {
        Self::get_lucky(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
