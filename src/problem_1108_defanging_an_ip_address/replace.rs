pub struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace('.', "[.]")
    }
}

impl super::Solution for Solution {
    fn defang_i_paddr(address: String) -> String {
        Self::defang_i_paddr(address)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
