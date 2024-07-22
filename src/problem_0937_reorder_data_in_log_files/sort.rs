pub struct Solution;

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut letter = Vec::with_capacity(logs.len());
        let mut digit = Vec::with_capacity(logs.len());
        for log in logs {
            let (_, rest) = log.split_once(' ').unwrap();
            if rest.as_bytes()[0].is_ascii_lowercase() {
                letter.push(log);
            } else {
                digit.push(log);
            }
        }
        letter.sort_unstable_by(|x, y| {
            let (ident1, rest1) = x.split_once(' ').unwrap();
            let (ident2, rest2) = y.split_once(' ').unwrap();
            match rest1.cmp(rest2) {
                std::cmp::Ordering::Equal => ident1.cmp(ident2),
                ordering => ordering,
            }
        });
        [letter, digit].concat()
    }
}

impl super::Solution for Solution {
    fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        Self::reorder_log_files(logs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
