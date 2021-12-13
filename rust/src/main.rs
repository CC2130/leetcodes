fn main() {
    println!("Hello, world!");
    test();
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }

  fn to_vec(mut lns: Option<Box<Self>>) -> Vec<Self> {
      let mut v = vec![];
      while let Some(mut ln) = lns {
          lns = ln.next.take();
          v.push(*ln);
      }

      v
  }

  fn fv(mut v: Vec<Self>) -> Option<Box<Self>> {
      let mut head = None;
      while let Some(mut tail) = v.pop() {
          tail.next = head;
          head = Some(Box::new(tail));
      }

      head
  }

  fn from_vec(mut v: Vec<i32>) -> Option<Box<ListNode>> {
      let mut head;
      let mut tail = None;
      loop {
          if let Some(n) = v.pop() {
              head = ListNode::new(n);
              head.next = tail;
              tail = Some(Box::new(head));
          } else {
              break
          }
      }

      tail
    }
}

use std::cmp::Ordering;

//impl PartialEq for ListNode {
//    fn eq(&self, other: &Self) -> bool {
//        self.val == other.val
//    }
//}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut tail = &mut head;
    for i in 0..k {
        if let Some(node) = tail {
            tail = &mut node.next;
        } else {
            println!("a");
            return head
        }
    }

    let others = tail.take();
    let mut tail = reverse_k_group(others, k);

    while let Some(mut header) = head.take() {
        let others = header.next.take();
        header.next = tail;
        tail = Some(header);
        head = others;
    }

    //let mut tail = reverse_k_group(*tail, k);
    tail
}

fn test() {
    //let l = ListNode::from_vec(vec![0, 1, 2, 3, 4, 5, 6]);
    //let r = reverse_k_group(l, 3);
    let l = ListNode::from_vec(vec![9, 6, 3, 5, 8, 1, 2, 4, 7]);
    let mut r = ListNode::to_vec(l);
    println!("{:?}", r);
    r.sort();
    println!("{:?}", r);
    let l = ListNode::fv(r);
    println!("{:?}", l);
}