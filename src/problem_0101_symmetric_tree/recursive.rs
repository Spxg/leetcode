pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        #[allow(clippy::ref_option)]
        fn helper(
            left: &Option<Rc<RefCell<TreeNode>>>,
            right: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (left, right) {
                (None, None) => true,
                (Some(left), Some(right)) if left.borrow().val == right.borrow().val => {
                    helper(&left.borrow().left, &right.borrow().right)
                        && helper(&left.borrow().right, &right.borrow().left)
                }
                _ => false,
            }
        }
        root.map_or(true, |root| {
            helper(&root.borrow().left, &root.borrow().right)
        })
    }
}

impl super::Solution for Solution {
    fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_symmetric(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
