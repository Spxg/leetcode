pub struct Solution;

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        pairs.sort_unstable_by_key(|x| x[1]);

        let mut result = 0;
        let mut min = i32::MIN;

        for pair in pairs {
            if pair[0] > min {
                result += 1;
                min = pair[1];
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        Self::find_longest_chain(pairs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
