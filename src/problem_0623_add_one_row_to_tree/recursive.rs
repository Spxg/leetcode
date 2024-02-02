pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(root: &Rc<RefCell<TreeNode>>, depth: i32, val: i32, count: i32) {
            if count == depth {
                let left = root.borrow_mut().left.take();
                let right = root.borrow_mut().right.take();
                root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left,
                    right: None,
                })));
                root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: None,
                    right,
                })));
                return;
            }
            match (&root.borrow().left, &root.borrow().right) {
                (None, None) => (),
                (Some(node), None) | (None, Some(node)) => helper(node, depth, val, count + 1),
                (Some(left), Some(right)) => {
                    helper(left, depth, val, count + 1);
                    helper(right, depth, val, count + 1);
                }
            }
        }

        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })));
        }

        if let Some(node) = root.as_ref() {
            helper(node, depth, val, 2);
        }

        root
    }
}

impl super::Solution for Solution {
    fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::add_one_row(root, val, depth)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
