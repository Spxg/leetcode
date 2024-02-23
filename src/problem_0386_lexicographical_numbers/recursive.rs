pub struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        fn helper(start: i32, n: i32, result: &mut Vec<i32>) {
            for num in (start * 10)..(n + 1).min((start + 1) * 10) {
                result.push(num);
                helper(num, n, result);
            }
        }

        let mut result = vec![];
        for num in 1..10.min(n + 1) {
            result.push(num);
            helper(num, n, &mut result);
        }
        result
    }
}

impl super::Solution for Solution {
    fn lexical_order(n: i32) -> Vec<i32> {
        Self::lexical_order(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
