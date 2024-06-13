pub struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut iter = word.chars();
        let first = iter.next().unwrap();
        if first.is_ascii_uppercase() {
            iter.next().map_or(true, |n| {
                if n.is_ascii_uppercase() {
                    iter.all(|x| x.is_ascii_uppercase())
                } else {
                    iter.all(|x| x.is_ascii_lowercase())
                }
            })
        } else {
            iter.all(|x| x.is_ascii_lowercase())
        }
    }
}

impl super::Solution for Solution {
    fn detect_capital_use(word: String) -> bool {
        Self::detect_capital_use(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
