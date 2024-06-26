pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.chars()
            .rev()
            .skip_while(|x| x.is_whitespace())
            .take_while(|x| !x.is_whitespace())
            .count() as _
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
