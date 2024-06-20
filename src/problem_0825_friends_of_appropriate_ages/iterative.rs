pub struct Solution;

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut ages = ages;
        ages.sort_unstable_by(|a, b| b.cmp(a));

        let mut slice = ages.as_slice();
        let mut prev = None;
        while let Some((&x, rest)) = slice.split_first() {
            let count = match prev {
                Some((age, count)) if x == age => count,
                _ => rest.partition_point(|&y| y > x / 2 + 7) as i32,
            };
            result += count;
            prev = Some((x, count));
            slice = rest;
        }
        result
    }
}

impl super::Solution for Solution {
    fn num_friend_requests(ages: Vec<i32>) -> i32 {
        Self::num_friend_requests(ages)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
