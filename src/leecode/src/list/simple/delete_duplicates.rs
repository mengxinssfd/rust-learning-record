use crate::list::list_node::ListNode;
use crate::solution::Solution;

/// 21. 合并两个有序链表 // https://leetcode-cn.com/problems/merge-two-sorted-lists/
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = &mut head;
        while current.is_some() && current.as_ref().unwrap().next.is_some() {
            let c = current.as_ref().unwrap();
            if c.val == c.next.as_ref().unwrap().val {
                *current = current.as_mut().unwrap().next.take();
            } else {
                current = &mut current.as_mut().unwrap().next;
            }
        }
        head
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::list::list_node::ListNode;

    #[test]
    fn delete_duplicates() {
        let list1 = ListNode::from_3(&[1, 1, 2, 3, 3]);
        let list2 = Solution::delete_duplicates(list1);
        assert_eq!(list2.unwrap().to_list(), vec![1, 2, 3]);
    }
}
