pub struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let mut s = s.into_bytes();
        for ch in &mut s {
            if *ch >= b'A' && *ch <= b'Z' {
                *ch = *ch - b'A' + b'a';
            }
        }
        String::from_utf8(s).unwrap()
    }
}

impl super::Solution for Solution {
    fn to_lower_case(str: String) -> String {
        Self::to_lower_case(str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
