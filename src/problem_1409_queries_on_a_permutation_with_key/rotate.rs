pub struct Solution;

impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(queries.len());
        let mut nums: Vec<i32> = (1..=m).collect();
        for query in queries {
            let pos = nums.iter().position(|&x| x == query).unwrap();
            result.push(pos as i32);
            nums[0..=pos].rotate_right(1);
        }
        result
    }
}

impl super::Solution for Solution {
    fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        Self::process_queries(queries, m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
