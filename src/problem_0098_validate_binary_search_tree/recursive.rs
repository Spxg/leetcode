pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        #[allow(clippy::ref_option)]
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, last: &mut Option<i32>) -> bool {
            node.as_ref().is_none_or(|node| {
                helper(&node.borrow().left, last)
                    && last
                        .replace(node.borrow().val)
                        .is_none_or(|x| x < node.borrow().val)
                    && helper(&node.borrow().right, last)
            })
        }
        helper(&root, &mut None)
    }
}

impl super::Solution for Solution {
    fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
