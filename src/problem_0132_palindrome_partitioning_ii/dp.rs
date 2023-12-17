pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        fn is_palindrome(s: &[u8]) -> bool {
            let mut iter = s.iter();
            while let Some(front) = iter.next() {
                if let Some(back) = iter.next_back() {
                    if !front.eq(back) {
                        return false;
                    }
                } else {
                    break;
                }
            }

            true
        }

        fn helper(s: &[u8], range: (usize, usize), map: &mut HashMap<(usize, usize), i32>) -> i32 {
            if let Some(&val) = map.get(&range) {
                return val;
            }

            let (start, end) = range;

            let cut = if is_palindrome(&s[start..end]) {
                0
            } else {
                let mut min = (end - start - 1) as i32;
                for idx in start + 1..end {
                    if s[idx] == s[end - 1] && is_palindrome(&s[idx..end]) {
                        min =
                            min.min(helper(s, (start, idx), map) + helper(s, (idx, end), map) + 1);
                    }
                }
                min
            };

            map.insert(range, cut);
            cut
        }
        let s = s.as_bytes();

        helper(s, (0, s.len()), &mut HashMap::new())
    }
}

impl super::Solution for Solution {
    fn min_cut(s: String) -> i32 {
        Self::min_cut(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
