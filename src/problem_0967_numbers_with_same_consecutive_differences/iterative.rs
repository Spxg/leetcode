pub struct Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut result = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        for _ in 0..n - 1 {
            let mut temp = vec![];
            for num in result {
                let last = num % 10;
                if k == 0 {
                    temp.push(num * 10 + last);
                } else {
                    if last - k >= 0 {
                        temp.push(num * 10 + (last - k));
                    }
                    if last + k <= 9 {
                        temp.push(num * 10 + (last + k));
                    }
                }
            }
            result = temp;
        }

        result
    }
}

impl super::Solution for Solution {
    fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        Self::nums_same_consec_diff(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
