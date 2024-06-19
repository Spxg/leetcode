pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut min = usize::MAX;
        let mut result = vec![];
        let map = list1
            .into_iter()
            .enumerate()
            .map(|x| (x.1, x.0))
            .collect::<HashMap<_, _>>();
        for (idx, restaurant) in list2.into_iter().enumerate() {
            if let Some(&x) = map.get(&restaurant) {
                match (idx + x).cmp(&min) {
                    std::cmp::Ordering::Less => result = vec![restaurant],
                    std::cmp::Ordering::Equal => result.push(restaurant),
                    std::cmp::Ordering::Greater => continue,
                }
                min = (idx + x).min(min);
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        Self::find_restaurant(list1, list2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
