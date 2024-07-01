pub struct Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut result = Vec::with_capacity(operations.len());
        for operation in operations {
            let len = result.len();
            match operation.as_str() {
                "+" => result.push(result[len - 2] + result[len - 1]),
                "D" => result.push(result[len - 1] * 2),
                "C" => {
                    result.pop();
                }
                num => result.push(num.parse::<i32>().unwrap()),
            }
        }
        result.into_iter().sum()
    }
}

impl super::Solution for Solution {
    fn cal_points(ops: Vec<String>) -> i32 {
        Self::cal_points(ops)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
