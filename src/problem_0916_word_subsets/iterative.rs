pub struct Solution;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut result = Vec::with_capacity(words1.len());

        let mut target = [0; 26];
        for word2 in words2 {
            let mut chs = [0; 26];
            for ch in word2.bytes() {
                let idx = (ch - b'a') as usize;
                chs[idx] += 1;
                target[idx] = target[idx].max(chs[idx]);
            }
        }
        for word1 in words1 {
            let mut chs = [0; 26];
            word1.bytes().for_each(|x| chs[(x - b'a') as usize] += 1);
            if chs.into_iter().zip(target).all(|(x, y)| x >= y) {
                result.push(word1);
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        Self::word_subsets(words1, words2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
