pub struct Solution;

/// for each node we check if we still have empty slots to put it in.
///
/// a null node occupies one slot.
///
/// a non-null node occupies one slot before he creates two more. the net gain is one.
impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut slot = 1;
        for ch in preorder.split(',') {
            if slot == 0 {
                return false;
            }
            if ch == "#" {
                slot -= 1;
            } else {
                slot += 1;
            }
        }
        slot == 0
    }
}

impl super::Solution for Solution {
    fn is_valid_serialization(preorder: String) -> bool {
        Self::is_valid_serialization(preorder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
