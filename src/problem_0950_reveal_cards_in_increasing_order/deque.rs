pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut result = VecDeque::with_capacity(deck.len());
        let mut deck = deck;
        deck.sort_unstable();
        for num in deck.into_iter().rev() {
            if let Some(n) = result.pop_back() {
                result.push_front(n);
            }
            result.push_front(num);
        }
        result.into()
    }
}

impl super::Solution for Solution {
    fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        Self::deck_revealed_increasing(deck)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
