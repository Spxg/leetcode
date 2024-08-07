pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn helper(
            root1: &Option<Rc<RefCell<TreeNode>>>,
            root2: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (root1, root2) {
                (None, None) => true,
                (None, Some(_)) | (Some(_), None) => false,
                (Some(node1), Some(node2)) => {
                    node1.borrow().val == node2.borrow().val
                        && (helper(&node1.borrow().left, &node2.borrow().left)
                            || helper(&node1.borrow().left, &node2.borrow().right))
                        && (helper(&node1.borrow().right, &node2.borrow().right)
                            || helper(&node1.borrow().right, &node2.borrow().left))
                }
            }
        }
        helper(&root1, &root2)
    }
}

impl super::Solution for Solution {
    fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::flip_equiv(root1, root2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
