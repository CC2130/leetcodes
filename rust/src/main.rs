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
    let l = ListNode::from_vec(vec![0, 1, 2, 3, 4, 5, 6]);
    let r = reverse_k_group(l, 3);
    println!("{:?}", r);
}