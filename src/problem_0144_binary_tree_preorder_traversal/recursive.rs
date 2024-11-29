pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        #[allow(clippy::ref_option)]
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(node) = root {
                result.push(node.borrow().val);
                helper(&node.borrow().left, result);
                helper(&node.borrow().right, result);
            }
        }
        let mut result = vec![];
        helper(&root, &mut result);
        result
    }
}

impl super::Solution for Solution {
    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::preorder_traversal(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
