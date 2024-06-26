pub mod math;

pub trait Solution {
    fn add_binary(a: String, b: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("11", "1"), "100"),
            (("1010", "1011"), "10101"),
            (("101111", "10"), "110001"),
            (("0", "0"), "0"),
        ];

        for ((a, b), expected) in test_cases {
            assert_eq!(S::add_binary(a.to_string(), b.to_string()), expected);
        }
    }
}
