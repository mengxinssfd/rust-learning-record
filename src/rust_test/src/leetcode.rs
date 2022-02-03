#![allow(unused)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Index;
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

        pub fn reverse_prefix(word: String, ch: char) -> String {
            // let i = word.chars().position(|c| c == ch);
            let i = word.find(ch);

            match i {
                None => word,
                Some(i) => {
                    let prefix = (&word[..i + 1]).chars().rev().collect::<String>();
                    let affix = &word[i + 1..];
                    prefix + affix
                }
            }
        }

        /// 67. 二进制求和
        pub fn add_binary(a: String, b: String) -> String {
            let mut len1 = (a.len() - 1) as i32;
            let mut len2 = (b.len() - 1) as i32;

            let mut carry = 0;
            let mut res = String::from("");

            let a = a.as_bytes();
            let b = b.as_bytes();

            while len1 > -1 || len2 > -1 {
                let num1 = *a.get(len1 as usize).unwrap_or(&48) - 48;
                let num2 = *b.get(len2 as usize).unwrap_or(&48) - 48;

                let mut val = num1 + num2 + carry;
                carry = (val / 2) as u8;
                val = val % 2;
                res.insert(0, (val + 48) as char);

                len1 -= 1;
                len2 -= 1;
            }

            if carry > 0 {
                res.insert(0, 49 as char);
            }

            res
        }

        /// 389. 找不同  // https://leetcode-cn.com/problems/find-the-difference/
        pub fn find_the_difference(s: String, t: String) -> char {
            let mut arr: [i8; 26] = [0; 26];
            let a = 'a' as usize;
            s.chars().for_each(|c| arr[(c as usize) - a] += 1);
            t.chars().for_each(|c| arr[(c as usize) - a] -= 1);
            let i = arr.iter().position(|&v| v == -1).unwrap();
            (a as u8 + i as u8) as char
        }

        pub fn majority_element(nums: Vec<i32>) -> i32 {
            let mut map = HashMap::new();
            nums.iter().for_each(|&n| {
                map.insert(n, map.get(&n).unwrap_or(&0) + 1);
            });
            let half = nums.len() / 2;
            *map.iter().find(|(_, &v)| v > half).unwrap().0 as i32
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::Solution;
    #[test]
    fn majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }
    #[test]
    fn find_the_difference() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
    }
    #[test]
    fn add_binary() {
        assert_eq!(
            Solution::add_binary("1111".to_string(), "1111".to_string()),
            "11110".to_string()
        );
        let s = "12345".to_string();
        let len = s.len() - 1;
        let mut cs = s.chars();
        println!("{:?}", cs);
        for i in 0..len {
            println!("{:?}, {:?}", cs.nth(len - i), cs);
        }

        for &b in "12345".as_bytes() {
            println!("b {}", b - 48);
        }

        println!("bytes{:?}", "12345".as_bytes());

        let ten = "10".to_string();
        let one = "1".to_string();
        assert_eq!(one + &ten, "110".to_string());
        assert_eq!(ten.chars().nth(0).unwrap(), '1');
    }
    #[test]
    fn reverse_prefix() {
        assert_eq!(
            Solution::reverse_prefix("leetcode".to_string(), 't'),
            "teelcode".to_string()
        );
        assert_eq!(
            Solution::reverse_prefix("leetcode".to_string(), 'l'),
            "leetcode".to_string()
        );
        assert_eq!(
            Solution::reverse_prefix("leetcode".to_string(), 'h'),
            "leetcode".to_string()
        );
    }
    #[test]
    fn first_uniq_char() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);

        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }
}
