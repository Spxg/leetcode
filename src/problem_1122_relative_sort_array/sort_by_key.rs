pub struct Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut position = vec![usize::MAX; 1001];
        for (i, n) in arr2.into_iter().enumerate() {
            position[n as usize] = i;
        }
        arr1.sort_unstable_by_key(|&n| (position[n as usize], n));
        arr1
    }
}

impl super::Solution for Solution {
    fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        Self::relative_sort_array(arr1, arr2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
