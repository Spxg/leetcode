pub struct Solution;

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let matched = query
                .chars()
                .filter(char::is_ascii_uppercase)
                .eq(pattern.chars().filter(char::is_ascii_uppercase))
                && query
                    .split(|x: char| x.is_ascii_uppercase())
                    .zip(pattern.split(|x: char| x.is_ascii_uppercase()))
                    .all(|(query, pattern)| {
                        let mut query = query.chars();
                        pattern.chars().all(|x| query.any(|q| q == x))
                    });
            result.push(matched);
        }
        result
    }
}

impl super::Solution for Solution {
    fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        Self::camel_match(queries, pattern)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
