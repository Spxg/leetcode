pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut s = s.into_bytes();
        s.split_mut(|&ch| ch == b' ').for_each(<[u8]>::reverse);
        String::from_utf8(s).unwrap()
    }
}

impl super::Solution for Solution {
    fn reverse_words(s: String) -> String {
        Self::reverse_words(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
