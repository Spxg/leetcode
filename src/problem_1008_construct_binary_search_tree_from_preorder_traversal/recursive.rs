pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            let (&first, rest) = preorder.split_first()?;
            let idx = rest.iter().position(|&x| x > first).unwrap_or(rest.len());
            Some(Rc::new(RefCell::new(TreeNode {
                val: first,
                left: helper(&rest[..idx]),
                right: helper(&rest[idx..]),
            })))
        }
        helper(&preorder)
    }
}

impl super::Solution for Solution {
    fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::bst_from_preorder(preorder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
