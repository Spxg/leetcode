pub struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let _ = n;
        let mut result = vec![];
        let mut n = 1;
        for num in target {
            while num != n {
                result.push("Push".into());
                result.push("Pop".into());
                n += 1;
            }
            result.push("Push".into());
            n += 1;
        }
        result
    }
}

impl super::Solution for Solution {
    fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        Self::build_array(target, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
