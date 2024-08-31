pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut map = HashSet::with_capacity(emails.len());
        for email in emails {
            let (name, domain) = email.split_once('@').unwrap();
            let mut s = String::new();
            for ch in name.chars() {
                match ch {
                    '.' => continue,
                    '+' => break,
                    other => s.push(other),
                }
            }
            map.insert(format!("{s}@{domain}"));
        }
        map.len() as _
    }
}

impl super::Solution for Solution {
    fn num_unique_emails(emails: Vec<String>) -> i32 {
        Self::num_unique_emails(emails)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
