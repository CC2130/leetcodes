// Definition for singly-linked list.
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

  fn boxed(self) -> Option<Box<ListNode>> {
      Some(Box::new(self))
  }

  fn from_vec(v: &mut Vec<ListNode>) -> Self {
      let mut res = None;
      loop {
          if let Some(mut next) = v.pop() {
              next.next = res;
              res = next.boxed();
          } else {
              break
          }
      }

      *res.unwrap()
  }

  fn from_num_vec(v: &Vec<i32>) -> Self {
      let mut lv = Vec::new();
      for i in v {
          lv.push(ListNode::new(*i));
      }

      ListNode::from_vec(&mut lv)
  }

  fn to_vec(self) -> Vec<ListNode> {
      fn _to_vec(mut lns: ListNode, v: &mut Vec<ListNode>) {
          if let Some(next) = lns.next {
              lns.next = None;
              v.push(lns);
              _to_vec(*next, v);
          } else {
              v.push(lns);
          }
      }

      let mut v = Vec::new();
      _to_vec(self, &mut v);
      v
  }

  // 将消耗自身
  fn to_num_vec(self) -> Vec<i32> {
      let mut v = Vec::new();
      for i in self.to_vec() {
          v.push(i.val);
      }

      v
  }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(l1), Some(l2)) => {
            let mut lns1 = l1.to_vec();
            let mut lns2 = l2.to_vec();
            let mut val;
            let mut ten = 0;
            let mut v = Vec::new();
            loop {
                match (lns1.pop(), lns2.pop()) {
                    (Some(ln1), Some(ln2)) => {
                        val = ln1.val + ln2.val + ten;
                        ten = 0;
                        if val >= 10 {
                            val -= 10;
                            ten = 1;
                        }
                        v.insert(0, ListNode::new(val));
                    },
                    (Some(ln), None) | (None, Some(ln)) => {
                        val = ln.val + ten;
                        ten = 0;
                        if val >= 10 {
                            val -= 10;
                            ten = 1;
                        }
                        v.insert(0, ListNode::new(val));
                    },
                    (None, None) => {
                        if ten > 0 {
                            v.insert(0, ListNode::new(1));
                        }

                        return ListNode::from_vec(&mut v).boxed()
                    }
                }
            }
        },
        (Some(n), None) | (None, Some(n)) => {
            Some(n)
        },
        _ => {
            None
        }
    }
}

// l1 = [7,2,4,3], l2 = [5,6,4]
// Output: [7,8,0,7]
fn test(v1: Vec<i32>, v2: Vec<i32>) {
    let l1 = ListNode::from_num_vec(&v1).boxed();
    let l2 = ListNode::from_num_vec(&v2).boxed();
    let res = add_two_numbers(l1, l2).unwrap().to_num_vec();
    println!("l1: {:?}\nl2: {:?}", v1, v2);
    println!("res: {:?}", res);
}

fn run() {
    println!("test 1:");
    let mut v1 = vec![7, 2, 4, 3];
    let mut v2 = vec![5, 6, 4];
    test(v1, v2);
    println!("\n\n");

    println!("test 2:");
    v1 = vec![9, 9, 9, 9];
    v2 = vec![5, 6, 4];
    test(v1, v2);
    println!("\n\n");

    println!("test 3:");
    v1 = vec![2, 3, 4];
    v2 = vec![5, 6, 4];
    test(v1, v2);
}

fn main() {
    println!("Hello, world!");
    run();
}
