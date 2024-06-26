pub mod dp;
pub mod recursive;

pub trait Solution {
    fn coin_change(coins: Vec<i32>, amount: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 5] as &[_], 11), 3), ((&[2], 3), -1)];

        for ((coins, amount), expected) in test_cases {
            assert_eq!(S::coin_change(coins.to_vec(), amount), expected);
        }
    }
}
