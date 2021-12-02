/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        let mut caculator = (l1, l2, 0, 0);

        loop {
            caculator = match caculator {
                (Some(l1), Some(l2), add, _) => {
                    if l1.val + l2.val + add >= 10 {
                        (l1.next, l2.next, 1, l1.val + l2.val + add - 10)
                    } else {
                        (l1.next, l2.next, 0, l1.val + l2.val + add)
                    }
                },
                (Some(l), None, add, _) | (None, Some(l), add, _) => {
                    if l.val + add >= 10 {
                        (l.next, None, 1, l.val + add - 10)
                    } else {
                        (l.next, None, 0, l.val + add)
                    }
                },
                (None, None, add, _) => {
                    if add > 0 {
                        (None, None, 0, 1)
                    } else {
                        break
                    }
                }
            };

            *tail = Some(Box::new(ListNode::new(caculator.3)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }
}
// @lc code=end

