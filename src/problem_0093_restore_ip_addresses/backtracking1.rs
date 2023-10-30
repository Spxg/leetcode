pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn helper(s: &[i32], element: &mut Vec<String>, result: &mut Vec<String>) {
            if let Some((&first, rest)) = s.split_first() {
                element.push(first.to_string());
                helper(rest, element, result);
                element.pop();
                if first != 0 {
                    if let Some((&secend, last1)) = rest.split_first() {
                        element.push((first * 10 + secend).to_string());
                        helper(last1, element, result);
                        element.pop();
                        if let Some((&third, last2)) = last1.split_first() {
                            let num = first * 100 + secend * 10 + third;
                            if num <= 255 {
                                element.push(num.to_string());
                                helper(last2, element, result);
                                element.pop();
                            }
                        }
                    }
                }
            } else if element.len() == 4 {
                result.push(element.join("."));
            }
        }

        let s = s.as_bytes().iter().map(|x| i32::from(*x - b'0')).collect::<Vec<_>>();
        let mut result = vec![];
        helper(&s, &mut Vec::new(), &mut result);
        result
    }
}

impl super::Solution for Solution {
    fn restore_ip_addresses(s: String) -> Vec<String> {
        Self::restore_ip_addresses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
