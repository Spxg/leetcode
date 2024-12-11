pub struct Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        let mut letter = vec![];
        let mut digit = vec![];
        for ch in s.chars() {
            if ch.is_numeric() {
                digit.push(ch);
            } else {
                letter.push(ch);
            }
        }
        if letter.len().abs_diff(digit.len()) > 1 {
            return String::new();
        }
        let mut result = String::with_capacity(s.len());

        let (a, mut b) = if letter.len() > digit.len() {
            (letter.into_iter(), digit.into_iter())
        } else {
            (digit.into_iter(), letter.into_iter())
        };

        for ch in a {
            result.push(ch);
            if let Some(ch) = b.next() {
                result.push(ch);
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn reformat(s: String) -> String {
        Self::reformat(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
