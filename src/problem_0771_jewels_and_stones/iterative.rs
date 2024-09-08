pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut mark = [false; 128];
        jewels.bytes().for_each(|idx| mark[idx as usize] = true);
        stones
            .bytes()
            .fold(0, |acc, x| acc + i32::from(mark[x as usize]))
    }
}

impl super::Solution for Solution {
    fn num_jewels_in_stones(j: String, s: String) -> i32 {
        Self::num_jewels_in_stones(j, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
