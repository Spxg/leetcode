use crate::data_structures::TreeNode;

use std::{cell::RefCell, rc::Rc};

struct BSTIterator {
    node: Option<Rc<RefCell<TreeNode>>>,
}

fn helper2(node: &mut Rc<RefCell<TreeNode>>, root: Rc<RefCell<TreeNode>>) {
    if node.borrow().right.is_none() {
        node.borrow_mut().right = Some(root);
    } else {
        helper2(node.borrow_mut().right.as_mut().unwrap(), root);
    }
}

fn helper1(root: &mut Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let node = root.take().unwrap();
    let left = node.borrow_mut().left.take();

    if let Some(mut left) = left {
        helper2(&mut left, node);
        *root = Some(left);
        helper1(root)
    } else {
        *root = node.borrow_mut().right.take();
        node.borrow().val
    }
}

impl BSTIterator {
    fn new(node: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self { node }
    }

    fn next(&mut self) -> i32 {
        helper1(&mut self.node)
    }

    fn has_next(&self) -> bool {
        self.node.is_some()
    }
}

impl super::BSTIterator for BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self::new(root)
    }

    fn next(&mut self) -> i32 {
        self.next()
    }

    fn has_next(&self) -> bool {
        self.has_next()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::BSTIterator>();
    }
}
