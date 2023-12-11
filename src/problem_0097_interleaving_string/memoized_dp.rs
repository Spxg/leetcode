pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        fn helper(
            s1: &[u8],
            s2: &[u8],
            s3: &[u8],
            map: &mut HashMap<(usize, usize), bool>,
        ) -> bool {
            if let Some(result) = map.get(&(s1.len(), s2.len())) {
                return *result;
            }

            let result = if let Some((first, rest)) = s3.split_first() {
                s1.split_first()
                    .filter(|x| x.0 == first)
                    .map(|x| helper(x.1, s2, rest, map))
                    .unwrap_or_default()
                    || s2
                        .split_first()
                        .filter(|x| x.0 == first)
                        .map(|x| helper(s1, x.1, rest, map))
                        .unwrap_or_default()
            } else {
                true
            };

            map.insert((s1.len(), s2.len()), result);
            result
        }

        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let mut map = HashMap::new();
        helper(s1.as_bytes(), s2.as_bytes(), s3.as_bytes(), &mut map)
    }
}

impl super::Solution for Solution {
    fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        Self::is_interleave(s1, s2, s3)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
