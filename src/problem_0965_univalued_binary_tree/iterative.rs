pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();
        let val = root.borrow().val;
        let mut stack = vec![root.borrow().left.clone(), root.borrow().right.clone()];
        while let Some(node) = stack.pop() {
            if let Some(n) = node {
                if n.borrow().val != val {
                    return false;
                }
                stack.push(n.borrow().left.clone());
                stack.push(n.borrow().right.clone());
            }
        }
        true
    }
}

impl super::Solution for Solution {
    fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_unival_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
