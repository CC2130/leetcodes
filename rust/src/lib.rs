// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn append(&mut self, other: Self) {
        let mut tail = self;
        while tail.next != None {
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = Some(Box::new(other));
    }

    pub fn from_vec(mut v: Vec<i32>) -> Option<Box<Self>> {
        let mut head = None;
        while let Some(i) = v.pop() {
            let mut node = ListNode::new(i);
            node.next = head;
            head = Some(Box::new(node));
        }

        head
    }

    pub fn to_vec(mut self) -> Vec<Self> {
        let mut res = vec![];
        while let Some(node) = self.next.take() {
            res.push(self);
            self = *node;
        }

        res
    }
}