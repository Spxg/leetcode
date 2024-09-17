pub struct Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut new = Vec::with_capacity(arr.len());
        let mut iter = arr.iter();
        while new.len() != arr.len() {
            let val = iter.next().copied().unwrap();
            new.push(val);
            if val == 0 && new.len() != arr.len() {
                new.push(0);
            }
        }
        *arr = new;
    }
}

impl super::Solution for Solution {
    fn duplicate_zeros(arr: &mut Vec<i32>) {
        Self::duplicate_zeros(arr);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
