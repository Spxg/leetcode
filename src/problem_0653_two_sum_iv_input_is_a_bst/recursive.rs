pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        #[allow(clippy::ref_option)]
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, k: i32, set: &mut HashSet<i32>) -> bool {
            if let Some(node) = node {
                let val = node.borrow().val;
                if set.contains(&val) {
                    return true;
                }
                set.insert(k - val);
                return helper(&node.borrow().left, k, set) || helper(&node.borrow().right, k, set);
            }
            false
        }
        helper(&root, k, &mut HashSet::new())
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
