pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let mut result = String::new();
        let map = knowledge
            .iter()
            .map(|pair| (&pair[0], &pair[1]))
            .collect::<HashMap<_, _>>();

        let mut key = None;
        for ch in s.chars() {
            match ch {
                '(' => key = Some(String::new()),
                ')' => {
                    let key = key.take().unwrap();
                    result.push_str(map.get(&key).map_or("?", |s| s.as_str()));
                }
                ch => {
                    if let Some(key) = key.as_mut() {
                        key.push(ch);
                    } else {
                        result.push(ch);
                    }
                }
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        Self::evaluate(s, knowledge)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
