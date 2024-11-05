pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut result = Vec::with_capacity(names.len());
        let mut map = HashMap::with_capacity(names.len());
        for name in names {
            if let Some(suffix) = map.get(&name).copied() {
                for new_suffix in suffix.. {
                    let name_suffix = format!("{name}({new_suffix})");
                    if map.contains_key(&name_suffix) {
                        continue;
                    }
                    result.push(name_suffix.clone());
                    map.insert(name_suffix, 1);
                    *map.get_mut(&name).unwrap() = new_suffix;
                    break;
                }
            } else {
                result.push(name.clone());
                map.insert(name, 1);
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn get_folder_names(names: Vec<String>) -> Vec<String> {
        Self::get_folder_names(names)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
