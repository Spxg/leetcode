pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn helper(
            root: &Rc<RefCell<TreeNode>>,
            element: &mut Vec<String>,
            result: &mut Vec<String>,
        ) {
            element.push(root.borrow().val.to_string());
            match (&root.borrow().left, &root.borrow().right) {
                (None, None) => result.push(element.join("->")),
                (None, Some(node)) | (Some(node), None) => helper(node, element, result),
                (Some(left), Some(right)) => {
                    helper(left, element, result);
                    helper(right, element, result);
                }
            }
            element.pop();
        }

        let mut result = vec![];
        if let Some(root) = root {
            helper(&root, &mut Vec::new(), &mut result);
        }
        result
    }
}

impl super::Solution for Solution {
    fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        Self::binary_tree_paths(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
