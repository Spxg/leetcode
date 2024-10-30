pub struct Solution;

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        fn helper(mut a: i32, mut b: i32) -> bool {
            let mut remainer;
            loop {
                remainer = a % b;
                if remainer == 0 {
                    break;
                }
                a = b;
                b = remainer;
            }
            b == 1
        }

        let mut result = vec![];
        for x in 2..=n {
            for y in 1..x {
                if helper(x, y) {
                    result.push(format!("{y}/{x}"));
                }
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn simplified_fractions(n: i32) -> Vec<String> {
        Self::simplified_fractions(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
