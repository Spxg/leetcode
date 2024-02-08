pub struct Solution;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut priority = [26; 26];
        order
            .bytes()
            .enumerate()
            .for_each(|(idx, val)| priority[(val - b'a') as usize] = idx as i32);

        let mut s = s.as_bytes().to_vec();
        s.sort_by(|&a, &b| priority[(a - b'a') as usize].cmp(&priority[(b - b'a') as usize]));

        String::from_utf8(s).unwrap()
    }
}

impl super::Solution for Solution {
    fn custom_sort_string(order: String, s: String) -> String {
        Self::custom_sort_string(order, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
