pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut version1 = version1.split('.').map(|x| x.parse::<i32>().unwrap());
        let mut version2 = version2.split('.').map(|x| x.parse::<i32>().unwrap());

        loop {
            match (version1.next(), version2.next()) {
                (None, None) => break,
                (None, Some(v2)) => {
                    if v2 == 0 {
                        continue;
                    }
                    return -1;
                }
                (Some(v1), None) => {
                    if v1 == 0 {
                        continue;
                    }
                    return 1;
                }
                (Some(v1), Some(v2)) => match v1.cmp(&v2) {
                    std::cmp::Ordering::Less => return -1,
                    std::cmp::Ordering::Equal => continue,
                    std::cmp::Ordering::Greater => return 1,
                },
            };
        }

        0
    }
}

impl super::Solution for Solution {
    fn compare_version(version1: String, version2: String) -> i32 {
        Self::compare_version(version1, version2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
