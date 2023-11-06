pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn helper(cur: i32, n: i32, k: usize, element: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if element.len() == k {
                result.push(element.clone());
            }

            for i in cur..=n {
                element.push(i);
                helper(i + 1, n, k, element, result);
                element.pop();
            }
        }
        let mut result = vec![];
        helper(1, n, k as usize, &mut Vec::new(), &mut result);
        result
    }
}

impl super::Solution for Solution {
    fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        Self::combine(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
