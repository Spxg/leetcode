pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter()
            .reduce(|acc, e| {
                acc.chars()
                    .zip(e.chars())
                    .take_while(|(lhs, rhs)| lhs == rhs)
                    .map(|x| x.0)
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl super::Solution for Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        Self::longest_common_prefix(strs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
