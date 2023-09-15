pub mod cheating;
pub mod kmp;
pub mod windows;

pub trait Solution {
    fn str_str(haystack: String, needle: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("hello", "ll"), 2),
            (("aaaaa", "bba"), -1),
            (("mississippi", "issip"), 4),
            (("aabaaabaaac", "aabaaac"), 4),
            (("aaa", "aaaa"), -1),
            (("mississippi", "issipi"), -1),
            (("abbabaaaabbbaabaabaabbbaaabaaaaaabbbabbaabbabaabbabaaaaababbabbaaaaabbbbaaabbaaabbbbabbbbaaabbaaaaababbaababbabaaabaabbbbbbbaabaabaabbbbababbbababbaaababbbabaabbaaabbbba", "bbbbbbaa"), 118),
        ];

        for ((haystack, needle), expected) in test_cases {
            assert_eq!(S::str_str(haystack.to_string(), needle.to_string()), expected);
        }
    }
}
