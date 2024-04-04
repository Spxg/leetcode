pub struct Solution;

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        fn helper(k: i32) -> i32 {
            if k == 1 {
                return 0;
            } else if k == 2 {
                return 1;
            }
            let remain = k - 2i32.pow(f64::from(k).log2() as u32);
            i32::from(if remain == 0 {
                helper(k / 2) == 0
            } else {
                helper(remain) == 0
            })
        }
        let _ = n;
        helper(k)
    }
}

impl super::Solution for Solution {
    fn kth_grammar(n: i32, k: i32) -> i32 {
        Self::kth_grammar(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
