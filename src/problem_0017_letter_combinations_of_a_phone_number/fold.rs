pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map = HashMap::from([
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]);

        digits.chars().fold(
            if digits.is_empty() { vec![] } else { vec![String::new()] },
            |acc, x| {
                acc.iter()
                    .flat_map(|s| {
                        std::iter::repeat(s)
                            .zip(map[&x].chars())
                            .map(|(s, c)| format!("{s}{c}"))
                            .collect::<Vec<_>>()
                    })
                    .collect()
            },
        )
    }
}

impl super::Solution for Solution {
    fn letter_combinations(digits: String) -> Vec<String> {
        Self::letter_combinations(digits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
