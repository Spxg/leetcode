pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        #[derive(PartialEq, Eq)]
        enum Pattern {
            Character(u8),
            Prev(u8),
            All,
        }

        fn helper(s: &[u8], p: &[Pattern]) -> bool {
            if s.is_empty() && p.is_empty() {
                return true;
            }

            if let Some((pattern, prest)) = p.split_first() {
                if let Some((first, rest)) = s.split_first() {
                    match pattern {
                        Pattern::Character(ch) => {
                            if ch == first || ch == &b'.' {
                                helper(rest, prest)
                            } else {
                                false
                            }
                        }
                        Pattern::Prev(ch) => {
                            if ch == first {
                                helper(rest, p) || helper(s, prest) || helper(rest, prest)
                            } else {
                                helper(s, prest)
                            }
                        }
                        Pattern::All => helper(rest, p) || helper(s, prest) || helper(rest, prest),
                    }
                } else {
                    match pattern {
                        Pattern::Prev(_) | Pattern::All => helper(s, prest),
                        Pattern::Character(_) => false,
                    }
                }
            } else {
                false
            }
        }

        let mut patterns = vec![];
        for &ch in p.as_bytes() {
            if ch == b'*' {
                if let Some(Pattern::Character(prev)) = patterns.pop() {
                    let pattern = if prev == b'.' {
                        Pattern::All
                    } else {
                        Pattern::Prev(prev)
                    };

                    if Some(&pattern) != patterns.last() {
                        patterns.push(pattern);
                    }
                }
            } else {
                patterns.push(Pattern::Character(ch));
            }
        }

        helper(s.as_bytes(), &patterns)
    }
}

impl super::Solution for Solution {
    fn is_match(s: String, p: String) -> bool {
        Self::is_match(s, p)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
