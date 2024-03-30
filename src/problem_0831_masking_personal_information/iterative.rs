pub struct Solution;

impl Solution {
    pub fn mask_pii(s: String) -> String {
        if let Some((name, domain)) = s.to_lowercase().split_once('@') {
            format!(
                "{}*****{}@{}",
                &name[0..1],
                &name[name.len() - 1..name.len()],
                domain
            )
        } else {
            let v = s.chars().filter(|x| x.is_numeric()).collect::<Vec<_>>();
            let len = v.len();
            format!(
                "{}***-***-{}",
                if len > 10 {
                    format!("+{}-", "*".repeat(len - 10))
                } else {
                    String::new()
                },
                String::from_iter(&v[len - 4..])
            )
        }
    }
}

impl super::Solution for Solution {
    fn mask_pii(s: String) -> String {
        Self::mask_pii(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
