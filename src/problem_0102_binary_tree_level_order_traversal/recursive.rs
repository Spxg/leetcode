pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        #[allow(clippy::ref_option)]
        fn helpr(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Vec<i32>>, level: usize) {
            if let Some(node) = node {
                if result.len() < level + 1 {
                    result.push(vec![]);
                }
                result[level].push(node.borrow().val);
                helpr(&node.borrow().left, result, level + 1);
                helpr(&node.borrow().right, result, level + 1);
            }
        }
        let mut result = vec![];
        helpr(&root, &mut result, 0);
        result
    }
}

impl super::Solution for Solution {
    fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        Self::level_order(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
