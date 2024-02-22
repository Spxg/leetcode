pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set = HashSet::new();
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let val = node.borrow().val;
                if set.contains(&val) {
                    return true;
                }
                set.insert(k - val);
                stack.push(node.borrow_mut().left.take());
                stack.push(node.borrow_mut().right.take());
            }
        }
        false
    }
}

impl super::Solution for Solution {
    fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        Self::find_target(root, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
