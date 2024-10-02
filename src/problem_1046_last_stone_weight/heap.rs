pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = stones.into_iter().collect::<BinaryHeap<_>>();
        loop {
            break match (heap.pop(), heap.pop()) {
                (None, None) => 0,
                (None, Some(x)) | (Some(x), None) => x,
                (Some(y), Some(x)) => {
                    if x != y {
                        heap.push(y - x);
                    }
                    continue;
                }
            };
        }
    }
}

impl super::Solution for Solution {
    fn last_stone_weight(stones: Vec<i32>) -> i32 {
        Self::last_stone_weight(stones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
