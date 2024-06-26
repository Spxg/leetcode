pub mod math;

pub trait Solution {
    fn int_to_roman(num: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (3, "III"),
            (4, "IV"),
            (9, "IX"),
            (58, "LVIII"),
            (1994, "MCMXCIV"),
        ];

        for (num, expected) in test_cases {
            assert_eq!(S::int_to_roman(num), expected);
        }
    }
}
