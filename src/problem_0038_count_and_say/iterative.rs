pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = "1".to_string();
        for _ in 1..n {
            let mut next = String::new();
            let (mut len, mut last) = (0, '0');

            for ch in result.chars() {
                if last.eq(&ch) {
                    len += 1;
                } else {
                    if len != 0 {
                        next.push_str(&len.to_string());
                        next.push(last);
                    }
                    last = ch;
                    len = 1;
                }
            }
            next.push_str(&len.to_string());
            next.push(last);

            result = next;
        }
        result
    }
}

impl super::Solution for Solution {
    fn count_and_say(n: i32) -> String {
        Self::count_and_say(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
