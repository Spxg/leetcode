pub struct Solution;

impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        let (mut a, mut b) = (a, b);
        let mut result = String::with_capacity((a + b) as usize);

        while a + b != 0 {
            if result.ends_with("aa") {
                result.push('b');
                b -= 1;
            } else if result.ends_with("bb") || a >= b {
                result.push('a');
                a -= 1;
            } else {
                result.push('b');
                b -= 1;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn str_without3a3b(a: i32, b: i32) -> String {
        Self::str_without3a3b(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
