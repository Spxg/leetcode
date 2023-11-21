pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn helper(
            candidates: &[i32],
            target: i32,
            part: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if target == 0 {
                result.push(part.clone());
                return;
            }
            if let Some((&first, rest)) = candidates.split_first() {
                if target >= first {
                    part.push(first);
                    helper(candidates, target - first, part, result);
                    part.pop();
                }

                helper(rest, target, part, result);
            }
        }

        let mut result = Vec::new();
        helper(&candidates, target, &mut Vec::new(), &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::combination_sum(candidates, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
