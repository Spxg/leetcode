pub struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let mut merge = [i32::MAX; 26];

        for word in words {
            let mut chs = [0; 26];
            for idx in word.bytes().map(|x| (x - b'a') as usize) {
                chs[idx] += 1;
            }
            for idx in 0..26 {
                merge[idx] = merge[idx].min(chs[idx]);
            }
        }

        for (ch, count) in merge.into_iter().enumerate() {
            (0..count).for_each(|_| result.push(((ch as u8 + b'a') as char).to_string()));
        }

        result
    }
}

impl super::Solution for Solution {
    fn common_chars(words: Vec<String>) -> Vec<String> {
        Self::common_chars(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
