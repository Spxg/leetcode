pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn hepler(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            preorder.split_first().map(|(&root, rest)| {
                let pos = inorder.iter().position(|&x| x == root).unwrap();
                Rc::new(RefCell::new(TreeNode {
                    val: root,
                    left: hepler(&rest[0..pos], &inorder[0..pos]),
                    right: hepler(&rest[pos..], &inorder[pos + 1..]),
                }))
            })
        }
        hepler(&preorder, &inorder)
    }
}

impl super::Solution for Solution {
    fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree(preorder, inorder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
