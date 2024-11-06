pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let mut result = vec![];
        let mut map = HashMap::new();
        for (idx, companies) in favorite_companies.iter().enumerate() {
            for company in companies {
                map.entry(company).or_insert_with(HashSet::new).insert(idx);
            }
        }

        for (idx, companies) in favorite_companies.iter().enumerate() {
            let (first, rest) = companies.split_first().unwrap();
            let mut retain = map[&first].clone();
            for company in rest {
                retain = retain.intersection(&map[&company]).copied().collect();
            }
            if retain.len() == 1 {
                result.push(idx as i32);
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        Self::people_indexes(favorite_companies)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
