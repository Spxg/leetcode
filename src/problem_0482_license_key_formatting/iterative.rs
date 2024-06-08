pub struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let k = k as usize;
        let s = s
            .chars()
            .filter(|&x| (x != '-'))
            .map(|x| x.to_ascii_uppercase())
            .collect::<String>();
        s.as_bytes()
            .rchunks(k)
            .rev()
            .map(|x| String::from_utf8_lossy(x))
            .collect::<Vec<_>>()
            .join("-")
    }
}

impl super::Solution for Solution {
    fn license_key_formatting(s: String, k: i32) -> String {
        Self::license_key_formatting(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
