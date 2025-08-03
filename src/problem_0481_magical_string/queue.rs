pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let mut count = 3;
        let mut result = 1;
        let mut prev = 2;
        let mut queue = VecDeque::from([2]);

        while let Some(x) = queue.pop_front() {
            if count >= n {
                break;
            }
            count += x;
            prev = 1 + i32::from(prev == 1);
            queue.extend(std::iter::repeat_n(prev, x as usize));
            result += if count > n { 1 } else { x } * i32::from(prev == 1);
        }

        result
    }
}

impl super::Solution for Solution {
    fn magical_string(n: i32) -> i32 {
        Self::magical_string(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
