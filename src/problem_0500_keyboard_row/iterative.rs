pub struct Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut chs = [0; 128];
        let mut result = Vec::with_capacity(words.len());
        for (idx, row) in [
            "qwertyuiopQWERTYUIOP",
            "asdfghjklASDFGHJKL",
            "zxcvbnmZXCVBNM",
        ]
        .into_iter()
        .enumerate()
        {
            row.bytes().for_each(|x| chs[x as usize] = idx);
        }
        for word in words {
            let mut iter = word.bytes();
            let row = chs[iter.next().unwrap() as usize];
            if iter.all(|x| chs[x as usize] == row) {
                result.push(word);
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn find_words(words: Vec<String>) -> Vec<String> {
        Self::find_words(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
