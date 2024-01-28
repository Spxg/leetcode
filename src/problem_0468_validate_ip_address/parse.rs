pub struct Solution;

impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        fn helper_v4(query_ip: &str) -> bool {
            let ipv4 = query_ip.split('.').collect::<Vec<_>>();
            if ipv4.len() != 4 {
                return false;
            }
            for part in &ipv4 {
                if let Ok(n) = part.parse::<u8>() {
                    if n != 0 && part.starts_with('0') || n == 0 && part.len() != 1 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }

        fn helper_v6(query_ip: &str) -> bool {
            let ipv6 = query_ip.split(':').collect::<Vec<_>>();
            if ipv6.len() != 8 {
                return false;
            }
            for part in &ipv6 {
                if part.is_empty() || part.len() > 4 || part.chars().any(|c| !c.is_ascii_hexdigit())
                {
                    return false;
                }
            }
            true
        }

        if helper_v4(&query_ip) {
            "IPv4".into()
        } else if helper_v6(&query_ip) {
            "IPv6".into()
        } else {
            "Neither".into()
        }
    }
}

impl super::Solution for Solution {
    fn valid_ip_address(ip: String) -> String {
        Self::valid_ip_address(ip)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
