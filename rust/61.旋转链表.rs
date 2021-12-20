/*
 * @lc app=leetcode.cn id=61 lang=rust
 *
 * [61] 旋转链表
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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head == None || k == 0 {
            return head
        }

        let mut tail = &mut head;
        let mut i = 0;
        loop {
            if let Some(n) = tail {
                tail = &mut n.as_mut().next;
                i += 1;
            } else {
                break
            }
        }
        let k = i - k % i;
        i = 1;
        tail = &mut head;
        while i < k {
            if let Some(n) = tail {
                tail = &mut n.as_mut().next;
            }
            i += 1
        }

        let mut h = tail.as_mut().unwrap().next.take();
        tail = &mut h;

        loop {
            if let Some(n) = tail {
                tail = &mut n.as_mut().next;
            } else {
                *tail = head;
                break
            }
        }

        h
    }
}
// @lc code=end

