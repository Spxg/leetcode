pub struct Solution;

impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let s = s.into_bytes();
        let mut result = 0;
        let mut positions = Vec::with_capacity(s.len());
        let mut iter = s.iter().enumerate();
        let (mut pos, mut prev) = iter.next().unwrap();
        let mut count = 1;
        for (idx, val) in iter {
            if val == prev {
                count += 1;
            } else {
                positions.push((pos, count));
                prev = val;
                pos = idx;
                count = 1;
            }
        }
        positions.push((pos, count));

        for word in words {
            if word.len() > s.len() {
                continue;
            }

            let mut pos_iter = positions.iter();
            let mut iter = word.bytes().enumerate();
            let (mut prev_pos, mut prev_val) = iter.next().unwrap();

            while let Some((idx, count)) = pos_iter.next().copied() {
                if s[idx] != prev_val {
                    break;
                }
                let (next_pos, next_val) = iter
                    .find(|x| x.1 != prev_val)
                    .map_or((word.len(), None), |x| (x.0, Some(x.1)));
                let size = next_pos - prev_pos;
                if count >= 3 && size > count || count < 3 && size != count {
                    break;
                }
                if let Some(next_val) = next_val {
                    prev_pos = next_pos;
                    prev_val = next_val;
                } else if pos_iter.next().is_none() {
                    result += 1;
                } else {
                    break;
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn expressive_words(s: String, words: Vec<String>) -> i32 {
        Self::expressive_words(s, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
