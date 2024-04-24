pub struct Solution;

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut n = n;
        let mut stack = vec![];

        while n != 0 {
            let last = n % 10;
            n /= 10;
            if let Some(pos) = stack.iter().position(|&x| x > last) {
                let digit = stack[pos];
                stack[pos] = last;
                let mut result = n;
                for digit in std::iter::once(digit).chain(stack) {
                    if let Some(acc) = result
                        .checked_mul(10)
                        .and_then(|acc| acc.checked_add(digit))
                    {
                        result = acc;
                    } else {
                        return -1;
                    }
                }
                return result;
            }
            stack.push(last);
            stack.sort_unstable();
        }
        -1
    }
}

impl super::Solution for Solution {
    fn next_greater_element(n: i32) -> i32 {
        Self::next_greater_element(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
