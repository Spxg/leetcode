pub struct Solution;

impl Solution {
    pub fn find_replace_string(
        s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let s = s.into_bytes();
        let mut result = Vec::with_capacity(6000);

        let mut zip = indices
            .into_iter()
            .zip(sources)
            .zip(targets)
            .map(|((i, s), t)| ((i as usize, s.into_bytes()), t.into_bytes()))
            .collect::<Vec<_>>();
        zip.sort_unstable_by_key(|x| x.0 .0);

        let mut prev_idx = 0;
        for ((idx, source), target) in zip {
            if prev_idx > idx {
                continue;
            }
            result.extend_from_slice(&s[prev_idx..idx]);
            let end = source.len() + idx;
            if end <= s.len() && s[idx..end] == source {
                result.extend(target);
                prev_idx = end;
            } else {
                prev_idx = idx;
            }
        }
        result.extend_from_slice(&s[prev_idx..]);
        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn find_replace_string(
        s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        Self::find_replace_string(s, indices, sources, targets)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
