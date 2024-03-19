pub struct Solution;

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        fn helper(num: &str) -> (i32, i32) {
            let (real, imaginary) = num.split_once('+').unwrap();
            let imaginary = imaginary.split('i').next().unwrap();
            (real.parse().unwrap(), imaginary.parse().unwrap())
        }
        let (x1, y1) = helper(&num1);
        let (x2, y2) = helper(&num2);
        let real = x1 * x2 - y1 * y2;
        let imaginary = x1 * y2 + x2 * y1;
        format!("{real}+{imaginary}i")
    }
}

impl super::Solution for Solution {
    fn complex_number_multiply(a: String, b: String) -> String {
        Self::complex_number_multiply(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
