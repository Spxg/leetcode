pub mod dp;

pub trait Solution {
    fn generate_parenthesis(n: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, &["()"] as &[_]),
            (2, &["(())", "()()"]),
            (3, &["((()))", "(()())", "(())()", "()(())", "()()()"]),
        ];

        for (n, expected) in test_cases {
            let result = S::generate_parenthesis(n);
            for e in expected {
                assert!(result.iter().any(|x| x.eq(e)));
            }
        }
    }
}
