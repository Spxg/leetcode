pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let group_size = group_size as usize;
        if hand.len() % group_size != 0 {
            return false;
        }
        let mut map = HashMap::with_capacity(hand.len());
        for &num in &hand {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut map = map.into_iter().collect::<Vec<_>>();
        map.sort_unstable();

        for idx in 0..map.len() {
            let num = map[idx];
            if num.1 == 0 {
                continue;
            }
            for offset in 0..group_size {
                let target_idx = offset + idx;
                if target_idx >= map.len() {
                    return false;
                }
                let target = map[target_idx];
                if num.0 + offset as i32 != target.0 || target.1 < num.1 {
                    return false;
                }
                map[target_idx] = (target.0, target.1 - num.1);
            }
        }
        map.into_iter().all(|(_, count)| count == 0)
    }
}

impl super::Solution for Solution {
    fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        Self::is_n_straight_hand(hand, group_size)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
