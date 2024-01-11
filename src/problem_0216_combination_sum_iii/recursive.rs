pub struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn helper(k: usize, n: i32, element: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if n < 0 {
                return;
            }

            if element.len() == k && n == 0 {
                result.push(element.clone());
            }

            let start = element.last().copied().unwrap_or(0);
            for num in start + 1..=9 {
                element.push(num);
                helper(k, n - num, element, result);
                element.pop();
            }
        }

        let mut result = vec![];
        helper(k as usize, n, &mut Vec::new(), &mut result);
        result
    }
}

impl super::Solution for Solution {
    fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        Self::combination_sum3(k, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
