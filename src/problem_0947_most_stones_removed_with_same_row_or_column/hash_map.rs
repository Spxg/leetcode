pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        fn helper(map: &mut HashMap<(i32, i32), Vec<(i32, i32)>>, keys: Vec<(i32, i32)>) {
            for key in keys {
                if let Some(keys) = map.remove(&key) {
                    helper(map, keys);
                }
            }
        }

        let mut result = 0;
        let mut map = HashMap::with_capacity(2 * stones.len());
        for stone in &stones {
            map.entry((stone[0], -1))
                .or_insert(vec![])
                .push((-1, stone[1]));
            map.entry((-1, stone[1]))
                .or_insert(vec![])
                .push((stone[0], -1));
        }

        for stone in &stones {
            match (map.remove(&(stone[0], -1)), map.remove(&(-1, stone[1]))) {
                (None, None) => result += 1,
                (a, b) => {
                    for keys in a.into_iter().chain(b) {
                        helper(&mut map, keys);
                    }
                }
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        Self::remove_stones(stones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
