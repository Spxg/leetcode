pub struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut map = [false; 26];
        for ch in allowed.bytes() {
            map[(ch - b'a') as usize] = true;
        }
        words
            .into_iter()
            .filter(|x| x.bytes().all(|x| map[(x - b'a') as usize]))
            .count() as _
    }
}

impl super::Solution for Solution {
    fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        Self::count_consistent_strings(allowed, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
