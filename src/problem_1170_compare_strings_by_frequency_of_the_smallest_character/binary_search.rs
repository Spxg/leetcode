pub struct Solution;

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        fn helper(s: &str) -> i32 {
            let bytes = s.as_bytes();
            let Some((&first, rest)) = bytes.split_first() else {
                unreachable!();
            };
            let mut result = (first, 1_i32);
            for &ch in rest {
                match result.0.cmp(&ch) {
                    std::cmp::Ordering::Less => continue,
                    std::cmp::Ordering::Equal => result.1 += 1,
                    std::cmp::Ordering::Greater => result = (ch, 1),
                }
            }
            result.1
        }

        let mut f_words = Vec::with_capacity(words.len());
        words.into_iter().for_each(|x| f_words.push(helper(&x)));
        f_words.sort_unstable();

        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let frequency = helper(&query);
            let smaller = f_words.len() - f_words.partition_point(|&x| x <= frequency);
            result.push(smaller as i32);
        }

        result
    }
}

impl super::Solution for Solution {
    fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        Self::num_smaller_by_frequency(queries, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
