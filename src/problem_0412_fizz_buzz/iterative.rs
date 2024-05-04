pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = Vec::with_capacity(n as usize);
        for idx in 1..=n {
            result.push(match (idx % 3 == 0, idx % 5 == 0) {
                (true, true) => "FizzBuzz".into(),
                (true, false) => "Fizz".into(),
                (false, true) => "Buzz".into(),
                (false, false) => idx.to_string(),
            });
        }
        result
    }
}

impl super::Solution for Solution {
    fn fizz_buzz(n: i32) -> Vec<String> {
        Self::fizz_buzz(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
