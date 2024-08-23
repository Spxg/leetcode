pub struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut map = [0; 60];

        for time in time {
            let num = time % 60;
            result += map[((60 - num) % 60) as usize];
            map[num as usize] += 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        Self::num_pairs_divisible_by60(time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
