pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut result = vec![];
        let mut map = HashMap::new();
        for order in orders {
            let mut iter = order.into_iter().skip(1);
            let table = map.entry(iter.next().unwrap()).or_insert_with(HashMap::new);
            for food in iter {
                *table.entry(food).or_insert(0) += 1;
            }
        }
        let mut tables = map.keys().collect::<Vec<_>>();
        let mut foods = map
            .values()
            .flatten()
            .map(|(food, _)| food)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        tables.sort_unstable_by_key(|x| x.parse::<i32>().unwrap());
        foods.sort_unstable();

        result.push(
            std::iter::once("Table".to_string())
                .chain(foods.iter().map(|x| (*x).clone()))
                .collect(),
        );

        for table in tables {
            let mut part = vec![];
            part.push(table.clone());
            let table = &map[table];
            for food in &foods {
                part.push(table.get(*food).copied().unwrap_or(0).to_string());
            }
            result.push(part);
        }

        result
    }
}

impl super::Solution for Solution {
    fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        Self::display_table(orders)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
