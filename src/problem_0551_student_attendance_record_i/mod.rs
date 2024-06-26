pub mod iterative;

pub trait Solution {
    fn check_record(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("PPALLP", true),
            ("PPALLL", false),
            ("LLPLLPLLALLA", false),
            ("LALL", true),
            ("LLL", false),
            ("AA", false),
            ("ALA", false),
            ("LPLPLPLPLPL", true),
            ("LLPPPLPAPP", true),
            ("PPAPPLPPLP", true),
            ("P", true),
            ("LL", true),
            ("AL", true),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::check_record(s.to_string()), expected);
        }
    }
}
