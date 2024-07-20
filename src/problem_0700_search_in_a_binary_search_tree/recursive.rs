pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            root.and_then(|node| {
                let node_val = node.borrow().val;
                match val.cmp(&node_val) {
                    std::cmp::Ordering::Less => helper(node.borrow_mut().left.take(), val),
                    std::cmp::Ordering::Equal => Some(node),
                    std::cmp::Ordering::Greater => helper(node.borrow_mut().right.take(), val),
                }
            })
        }
        helper(root, val)
    }
}

impl super::Solution for Solution {
    fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::search_bst(root, val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
