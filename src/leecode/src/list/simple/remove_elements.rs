use crate::list::list_node::ListNode;
use crate::solution::Solution;

/// 203. 移除链表元素 // https://leetcode-cn.com/problems/remove-linked-list-elements/
impl Solution {
    // 递归
    pub fn remove_elements_by_recursion(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        head.as_mut().unwrap().next = Self::remove_elements_by_recursion(head.as_mut().unwrap().next.take(), val);
        if head.as_ref().unwrap().val == val {
            return head.as_mut().unwrap().next.take();
        }
        head
    }
    // 迭代
    /*pub fn remove_elements_by_iterator(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut h = Some(Box::new(ListNode::new(0)));
        // 使用'?'少写一个unwrap()
        h.as_mut()?.next = head;

        let mut head = &mut h;
        while head.as_ref()?.next.is_some() {
            if head.as_ref()?.next.as_ref()?.val == val {
                head.as_mut()?.next = head.as_mut()?.next.as_mut()?.next.take();
            } else {
                head = &mut head.as_mut()?.next;
            }
        }

        h.as_mut()?.next.take()
    }*/

    pub fn remove_elements_by_iterator(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut root = ListNode { val: 0, next: head };
        let mut head = &mut root;
        while let Some(ref mut cur) = head.next {
            if cur.val == val {
                head.next = cur.next.take();
            } else {
                head = head.next.as_mut()?;
            }
        }
        root.next
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::list::list_node::ListNode;

    #[test]
    fn test_label_question() {
        // 使用'?'必须所在函数返回Option或Result，可以少写一个unwrap()
        fn test(v: Vec<Option<i32>>) -> Option<i32> {
            for i in v {
                if i?.abs() == 1 {
                    println!("{:?}", i);
                }
            };
            Some(1)
        }
        assert_eq!(test(vec![Some(1i32), Some(-1), Some(-2)]), Some(1));
        assert_eq!(test(vec![Some(1i32), Some(-1), Some(-2), None]), None);
    }

    #[test]
    fn remove_elements() {
        // 递归
        let list1 = ListNode::from_3(&[1, 2, 6, 3, 4, 5, 6]);
        let list2 = Solution::remove_elements_by_recursion(list1, 6);
        assert_eq!(list2.unwrap().to_list(), vec![1, 2, 3, 4, 5]);

        let list1 = ListNode::from_3(&[7, 7, 7, 7]);
        let list2 = Solution::remove_elements_by_recursion(list1, 7);
        assert_eq!(list2, None);

        // 迭代
        let list1 = ListNode::from_3(&[1, 2, 6, 3, 4, 5, 6]);
        let list2 = Solution::remove_elements_by_iterator(list1, 6);
        assert_eq!(list2.unwrap().to_list(), vec![1, 2, 3, 4, 5]);

        let list1 = ListNode::from_3(&[7, 7, 7, 7]);
        let list2 = Solution::remove_elements_by_iterator(list1, 7);
        assert_eq!(list2, None);
    }
}
