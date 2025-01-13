pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut sum: i32 = piles.iter().sum();
        let mut heap = piles.into_iter().collect::<BinaryHeap<_>>();
        for _ in 0..k {
            if let Some(mut piles) = heap.pop() {
                sum -= piles / 2;
                piles -= piles / 2;
                heap.push(piles);
            }
        }
        sum
    }
}

impl super::Solution for Solution {
    fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        Self::min_stone_sum(piles, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
