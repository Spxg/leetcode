pub struct Solution;

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        s.split(' ').take(k as usize).collect::<Vec<_>>().join(" ")
    }
}

impl super::Solution for Solution {
    fn truncate_sentence(s: String, k: i32) -> String {
        Self::truncate_sentence(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
