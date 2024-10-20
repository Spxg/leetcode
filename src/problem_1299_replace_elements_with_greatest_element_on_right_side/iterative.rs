pub struct Solution;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; arr.len()];
        let mut max = -1;
        for idx in (0..arr.len()).rev() {
            result[idx] = max;
            max = arr[idx].max(max);
        }
        result
    }
}

impl super::Solution for Solution {
    fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        Self::replace_elements(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
