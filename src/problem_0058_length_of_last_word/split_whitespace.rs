pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().map(str::len).unwrap_or_default() as _
    }
}

impl super::Solution for Solution {
    fn length_of_last_word(s: String) -> i32 {
        Self::length_of_last_word(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
