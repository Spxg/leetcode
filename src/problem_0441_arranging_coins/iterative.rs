pub struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut result = 0;
        let mut n = n;
        for x in 1.. {
            n -= x;
            if n >= 0 {
                result += 1;
            } else {
                break;
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn arrange_coins(n: i32) -> i32 {
        Self::arrange_coins(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
