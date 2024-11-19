pub struct Solution;

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut result = vec![b'a'; n as usize];
        let mut k = k - n;
        for val in result.iter_mut().rev() {
            let remain = i32::from(b'z' - *val);
            if k >= remain {
                *val = b'z';
                k -= remain;
            } else {
                *val += k as u8;
                break;
            }
        }
        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn get_smallest_string(n: i32, k: i32) -> String {
        Self::get_smallest_string(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
