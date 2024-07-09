pub struct Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut result = Vec::with_capacity(words.len());
        'l: for word in words {
            let mut map1 = [-1; 26];
            let mut map2 = [-1; 26];
            for (source, target) in pattern.bytes().zip(word.bytes()) {
                let idx1 = (source - b'a') as usize;
                let idx2 = (target - b'a') as usize;
                if map1[idx1] == -1 && map2[idx2] == -1 {
                    map1[idx1] = i32::from(target);
                    map2[idx2] = i32::from(source);
                } else if map1[idx1] != i32::from(target) && map2[idx2] != i32::from(source) {
                    continue 'l;
                }
            }
            result.push(word);
        }
        result
    }
}

impl super::Solution for Solution {
    fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        Self::find_and_replace_pattern(words, pattern)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
