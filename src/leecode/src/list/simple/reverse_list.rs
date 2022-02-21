use crate::list::list_node::ListNode;
use crate::solution::Solution;

/// 206. 反转链表 // https://leetcode-cn.com/problems/reverse-linked-list/
impl Solution {
    // 递归
    /*pub fn reverse_list_by_recursion(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return if let Some(mut node) = head {
            if node.next.is_none() { return head.take(); }
            let mut next = node.next;
            let res = Self::reverse_list_by_recursion(next.take());

            res
        } else {
            head
        };
    }*/

    /*pub fn reverse_list_by_recursion(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let next = head.unwrap().next.take();
        let res = Self::reverse_list_by_recursion(next);
        next.as_mut().unwrap().next = head;
        res
    }*/
    pub fn reverse_list_by_recursion(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        pub fn recursion(cur: Option<Box<ListNode>>, pre: Box<ListNode>) -> Option<Box<ListNode>> {
            let mut cur = cur.unwrap();
            if cur.next.is_none() {
                return Some(cur);
            }

            let mut next = cur.next.replace(pre);//当前节点指向前一个节点，同时返回当前节点旧的下一个节点

            let end = recursion(next.take(), cur);
            return end;
        }

        if head.is_none() {
            return head;
        }

        let mut head = head.unwrap();
        if head.next.is_none() {
            return Some(head);
        }

        recursion(head.next.take(), head)
    }

    // 迭代
    pub fn reverse_list_by_iter(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
    pub fn reverse_list_by_iter2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while head.is_some() {
            let next = head.as_mut().unwrap().next.take();
            head.as_mut().unwrap().next = prev;
            prev = head.take();
            head = next;
        }
        prev
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::list::list_node::ListNode;

    #[test]
    fn test_replace() {
        let mut c = Some(1);
        let mut d = c.replace(2);
        let num = d.insert(3);
        println!("{}", num);
        println!("{:?},{:?}", c, d); // Some(2), Some(1)
    }

    #[test]
    fn reverse_list() {
        // 递归
        let list1 = ListNode::from_3(&[1, 2, 3, 4, 5]);
        let list2 = Solution::reverse_list_by_recursion(list1);
        // println!("{:#?}", list2);
        assert_eq!(list2.unwrap().to_list(), vec![5, 4, 3, 2, 1]);

        // 迭代
        let list1 = ListNode::from_3(&[1, 2, 3, 4, 5]);
        let list2 = Solution::reverse_list_by_iter(list1);
        assert_eq!(list2.unwrap().to_list(), vec![5, 4, 3, 2, 1]);
        // 迭代
        let list1 = ListNode::from_3(&[1, 2, 3, 4, 5]);
        let list2 = Solution::reverse_list_by_iter2(list1);
        assert_eq!(list2.unwrap().to_list(), vec![5, 4, 3, 2, 1]);
    }
}
