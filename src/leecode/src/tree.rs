use std::cell::RefCell;
use std::rc::Rc;
use crate::solution::Solution;


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(unused)]
impl Solution {

    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn is_same(
            p: &Option<Rc<RefCell<TreeNode>>>,
            q: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (p, q) {
                (Some(m), Some(n)) => {
                    let (mb, nb) = (m.borrow(), n.borrow());
                    mb.val == nb.val
                        && is_same(&mb.left, &nb.left)
                        && is_same(&mb.right, &nb.right)
                }
                (None, None) => true,
                _ => false,
            }
        }
        is_same(&p, &q)
    }
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(r) => {
                let left = r.borrow_mut().left.take();

                let right = r.borrow_mut().right.take();

                let mut val = 0;

                if left.is_some()
                    && left.as_ref().unwrap().borrow().left.is_none()
                    && left.as_ref().unwrap().borrow().right.is_none()
                {
                    val = left.as_ref().unwrap().borrow().val;
                }

                val + Self::sum_of_left_leaves(left) + Self::sum_of_left_leaves(right)
            }
            _ => 0,
        }
    }
}


#[cfg(test)]
mod test {

}