pub struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let bytes = s.to_lowercase().as_bytes().to_vec();
        let idxs = bytes
            .iter()
            .enumerate()
            .filter_map(|x| x.1.is_ascii_alphabetic().then_some(x.0))
            .collect::<Vec<_>>();

        let len = 2usize.pow(idxs.len() as u32);
        let mut result = Vec::with_capacity(len);
        result.push(bytes);

        for &idx1 in idxs.iter().rev() {
            for idx2 in 0..result.len() {
                let mut permutation = result[idx2].clone();
                permutation[idx1] = permutation[idx1].to_ascii_uppercase();
                result.push(permutation);
            }
        }

        result
            .into_iter()
            .map(|x| String::from_utf8(x).unwrap())
            .collect()
    }
}

impl super::Solution for Solution {
    fn letter_case_permutation(s: String) -> Vec<String> {
        Self::letter_case_permutation(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
