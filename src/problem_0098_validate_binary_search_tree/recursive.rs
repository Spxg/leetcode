pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, last: &mut Option<i32>) -> bool {
            node.as_ref().map_or(true, |node| {
                helper(&node.borrow().left, last)
                    && std::mem::replace(last, Some(node.borrow().val)).map_or(true, |x| x < node.borrow().val)
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
