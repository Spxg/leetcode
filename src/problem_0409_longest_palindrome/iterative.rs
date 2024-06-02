pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut chs = [0; 128];
        s.bytes().for_each(|x| chs[x as usize] += 1);

        let result = chs
            .into_iter()
            .fold(0, |acc, count| acc + (count - count % 2));
        result + i32::from(result as usize != s.len())
    }
}

impl super::Solution for Solution {
    fn longest_palindrome(s: String) -> i32 {
        Self::longest_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
