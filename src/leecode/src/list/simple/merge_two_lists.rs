use crate::list::list_node::ListNode;
use crate::solution::Solution;

/// 21. 合并两个有序链表 // https://leetcode-cn.com/problems/merge-two-sorted-lists/
impl Solution {
    // 递归
    pub fn merge_two_lists_by_recursion(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (l, None) => l,
            (None, l) => l,
            (Some(mut list1), Some(mut list2)) => {
                if list1.val < list2.val {
                    list1.next = Self::merge_two_lists_by_recursion(list1.next, Some(list2));
                    return Some(list1);
                }
                list2.next = Self::merge_two_lists_by_recursion(Some(list1), list2.next);
                return Some(list2);
            }
        }
    }
    // 迭代
    pub fn merge_two_lists_by_iterate(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut new_list = ListNode::new(0);
        let mut cur = &mut new_list;
        while list1.is_some() && list2.is_some() {
            let (l1, l2) = (list1.as_deref_mut().unwrap(), list2.as_deref_mut().unwrap());
            if l1.val < l2.val {
                let next = l1.next.take();
                cur.next = list1.take();
                list1 = next;
            } else {
                let next = l2.next.take();
                cur.next = list2.take();
                list2 = next;
            }
            cur = cur.next.as_deref_mut().unwrap();
        }
        cur.next = list1.or(list2);
        new_list.next
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::list::list_node::ListNode;

    #[test]
    fn merge_two_lists() {
        let list1 = ListNode::from_3(&[1, 2, 4]);
        let list2 = ListNode::from_3(&[1, 3, 4]);
        let list3 = Solution::merge_two_lists_by_recursion(list1, list2);
        assert_eq!(list3.unwrap().to_list(), vec![1, 1, 2, 3, 4, 4]);

        let list1 = ListNode::from_3(&[1, 2, 4]);
        let list2 = ListNode::from_3(&[1, 3, 4]);
        let list3 = Solution::merge_two_lists_by_iterate(list1, list2);
        assert_eq!(list3.unwrap().to_list(), vec![1, 1, 2, 3, 4, 4]);
    }
}
