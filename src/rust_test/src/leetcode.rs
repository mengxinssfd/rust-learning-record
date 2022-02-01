#![allow(unused)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Solution;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn tree() {
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

        pub fn first_uniq_char(s: String) -> i32 {
            let mut map = HashMap::new();
            /*  let mut i = 0;
            s.chars().for_each(|c| {
                 if map.contains_key(&c) {
                     map.insert(c, -1);
                 } else {
                     map.insert(c, i);
                 }
                 i += 1;
             });

             // println!("{:?}", map);

             for x in s.chars() {
                 match map.get(&x) {
                     Some(&v) if v != -1 => return v,
                     _ => {}
                 }
             }
             */

            /*
            for (i,c) in s.chars().enumerate() {
                if map.contains_key(&c) {
                    map.insert(c, -1);
                } else {
                    map.insert(c, i as i32);
                }
            };

            for x in s.chars() {
                if map[&x] != -1 {
                    return map[&x];
                }
            }
             */
            s.chars().for_each(|c| {
                *map.entry(c).or_insert(0) += 1;
            });

            println!("{:?}", map);
            for (i, c) in s.chars().enumerate() {
                if map[&c] == 1 {
                    return i as i32;
                }
            }
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::Solution;

    #[test]
    fn first_uniq_char() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);

        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }
}
