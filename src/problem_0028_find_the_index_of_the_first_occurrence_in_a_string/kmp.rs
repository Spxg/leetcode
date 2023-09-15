pub struct Solution;

impl Solution {
    fn move_map(s: &[u8]) -> Vec<usize> {
        let mut i = 0;
        let mut result = vec![0; s.len() + 1];

        for idx in 1..s.len() {
            while i > 0 && s[i] != s[idx] {
                i = result[i];
            }

            if s[i] == s[idx] {
                i += 1;
                result[idx + 1] = i;
            }
        }
        result
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let move_map = Self::move_map(needle);
        let mut idx = 0;

        if haystack.len() < needle.len() {
            return -1;
        }

        'lo: loop {
            for (lhs, rhs) in (idx..haystack.len()).zip(0..needle.len()) {
                if haystack.len() - idx < needle.len() {
                    break 'lo;
                }
                if haystack[lhs] != needle[rhs] {
                    if lhs == haystack.len() - 1 {
                        break 'lo;
                    }
                    let step = move_map[rhs];
                    idx += if step == 0 { 1 } else { rhs - step };
                    break;
                } else if needle.len() - 1 == rhs {
                    let result = lhs - needle.len() + 1;
                    return result as _;
                }
            }
        }
        -1
    }
}

impl super::Solution for Solution {
    fn str_str(haystack: String, needle: String) -> i32 {
        Self::str_str(haystack, needle)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
