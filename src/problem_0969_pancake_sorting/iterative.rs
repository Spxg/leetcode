pub struct Solution;

impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        let mut arr1 = arr;
        let mut arr2 = arr1.clone();
        arr2.sort_unstable();

        for (num, end) in arr2.into_iter().rev().zip((2..=arr1.len()).rev()) {
            let pos = arr1.iter().position(|&x| x == num).unwrap() + 1;
            arr1[0..pos].reverse();
            arr1[0..end].reverse();
            result.push(pos as i32);
            result.push(end as i32);
        }

        result
    }
}

impl super::Solution for Solution {
    fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        Self::pancake_sort(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
