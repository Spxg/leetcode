pub struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        if n < 10 {
            return n;
        }
        let n = n as u64;

        let mut left = 9;
        let mut size = 2;
        let mut start = 1;

        loop {
            let cap = size * 10;
            let right = left + cap * start * 9;
            if n > left && n <= right {
                let x = n - left;
                let rest = x % cap;
                let nth = rest % size;
                return if nth == 0 {
                    let result = rest / size;
                    if result == 0 {
                        9
                    } else {
                        result - 1
                    }
                } else {
                    let mut prefix = start + x / cap;
                    for _ in 0..size - nth - 1 {
                        prefix /= 10;
                    }
                    prefix % 10
                } as i32;
            }
            left = right;
            start *= 10;
            size += 1;
        }
    }
}

impl super::Solution for Solution {
    fn find_nth_digit(n: i32) -> i32 {
        Self::find_nth_digit(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
