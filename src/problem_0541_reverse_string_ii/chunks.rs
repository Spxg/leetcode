pub struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut s = s.into_bytes();
        for chunk in s.chunks_mut(2 * k) {
            let x = k.min(chunk.len());
            chunk[0..x].reverse();
        }
        String::from_utf8(s).unwrap()
    }
}

impl super::Solution for Solution {
    fn reverse_str(s: String, k: i32) -> String {
        Self::reverse_str(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
