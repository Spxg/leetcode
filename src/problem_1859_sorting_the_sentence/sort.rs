pub struct Solution;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut s = s.split(' ').collect::<Vec<_>>();
        s.sort_unstable_by_key(|x| x.chars().last());
        s.iter()
            .map(|&x| &x[0..x.len() - 1])
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl super::Solution for Solution {
    fn sort_sentence(s: String) -> String {
        Self::sort_sentence(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
