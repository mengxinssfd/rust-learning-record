// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct IntoIter {
    node: Option<Box<ListNode>>,
}

impl IntoIter {
    pub fn new(node: Box<ListNode>) -> IntoIter {
        IntoIter { node: Some(node) }
    }
}

impl Iterator for IntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.take() {
            let ListNode { next, val } = *node;
            self.node = next;
            return Some(val);
        }
        None
    }
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn into_iter(self) -> IntoIter {
        IntoIter::new(Box::new(self))
    }
    pub fn from(v: &Vec<i32>) -> Option<Box<Self>> {
        let mut head = Some(Box::new(Self::new(v[0])));
        let mut cur = &mut head;

        for &i in v.iter().skip(1) {
            let node = Box::new(Self::new(i));
            if let Some(x) = cur {
                x.next = Some(node);
                cur = &mut x.next;
            }
        }
        head
    }
    pub fn from_2(v: &Vec<i32>) -> Option<Box<Self>> {
        let mut head = None;
        let mut cur = &mut head;

        // 报错
        // `cur` declared here, outside of the closure body
        // v.into_iter().for_each(|&i| {
        //     cur = &mut cur.insert(Box::new(Self::new(i))).next;
        // });

        //
        for &i in v {
            cur = &mut cur.insert(Box::new(Self::new(i))).next;
        }
        head
    }
    pub fn from_3(v: &[i32]) -> Option<Box<Self>> {
        let mut head = None;
        let mut cur = &mut head;

        for &i in v {
            cur = &mut cur.insert(Box::new(Self::new(i))).next;
        }
        head
    }

    pub fn to_list(self) -> Vec<i32> {
        self.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::list::list_node::ListNode;

    #[test]
    fn test() {
        let mut node = Box::new(ListNode::new(0));
        let mut node1 = Box::new(ListNode::new(1));
        let node2 = Box::new(ListNode::new(2));
        node1.next = Some(node2);
        node.next = Some(node1);

        for item in node.clone().into_iter() {
            println!("iter {}", item);
        }

        println!("{:?}", node.clone().into_iter().collect::<Vec<i32>>());

        let mut test = Some(&node);

        while test.is_some() {
            println!("{}", test.unwrap().val);
            test = test.unwrap().next.as_ref();
        }
    }

    #[test]
    fn test_from() {
        let l = ListNode::from(&vec![1, 2, 3]);
        println!("{:?}", l);
        let l = ListNode::from_2(&vec![1, 2, 3]);
        println!("{:?}", l);
        let l = ListNode::from_3(&[1, 2, 3]);
        println!("{:?}", l);
        let l = ListNode::from_3(&[0, 1, 2]);
        println!("{:?}", l);
    }
    #[test]
    fn test_to() {
        let l = ListNode::from_3(&[1, 2, 3]).unwrap();
        assert_eq!(l.to_list(),vec![1,2,3])
    }



    #[test]
    fn test_none() {
        let mut n = None;
        let str = n.insert(String::from("hello"));
        str.push_str(" world");
        println!("{:?}", str);
        assert_eq!(str, "hello world");
        assert_eq!(n, Some("hello world".to_string()));

        let mut s = Some(1);
        let n = s.insert(20);
        *n += 30;
        assert_eq!(n.clone(), 50);
        assert_eq!(s, Some(50));
    }
}
