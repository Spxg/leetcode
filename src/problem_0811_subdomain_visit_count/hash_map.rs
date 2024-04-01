pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        for cpdomain in &cpdomains {
            let (count, mut domain) = cpdomain.split_once(' ').unwrap();
            let count = count.parse::<i32>().unwrap();
            *map.entry(domain).or_insert(0) += count;
            while let Some((_, parent)) = domain.split_once('.') {
                *map.entry(parent).or_insert(0) += count;
                domain = parent;
            }
        }
        map.into_iter().map(|(k, v)| format!("{v} {k}")).collect()
    }
}

impl super::Solution for Solution {
    fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        Self::subdomain_visits(cpdomains)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
