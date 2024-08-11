pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn helper(root: &Rc<RefCell<TreeNode>>, result: &mut Vec<i32>) {
            match (&root.borrow().left, &root.borrow().right) {
                (None, None) => result.push(root.borrow().val),
                (Some(node), None) | (None, Some(node)) => helper(node, result),
                (Some(left), Some(right)) => {
                    helper(left, result);
                    helper(right, result);
                }
            }
        }

        let root1 = root1.unwrap();
        let root2 = root2.unwrap();
        let mut result1 = vec![];
        let mut result2 = vec![];

        helper(&root1, &mut result1);
        helper(&root2, &mut result2);

        result1 == result2
    }
}

impl super::Solution for Solution {
    fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::leaf_similar(root1, root2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
