pub struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut current_max = arr[0];
        let mut result = 0;

        for (idx, x) in (0..).zip(arr) {
            current_max = current_max.max(x);
            if current_max == idx {
                result += 1;
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        Self::max_chunks_to_sorted(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
