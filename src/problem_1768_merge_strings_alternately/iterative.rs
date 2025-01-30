pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();
        for (x, y) in iter1
            .by_ref()
            .zip(iter2.by_ref())
            .take(word1.len().min(word2.len()))
        {
            result.push(x);
            result.push(y);
        }
        result.extend(iter1);
        result.extend(iter2);
        result
    }
}

impl super::Solution for Solution {
    fn merge_alternately(word1: String, word2: String) -> String {
        Self::merge_alternately(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
