/*
 * @lc app=leetcode.cn id=25 lang=rust
 *
 * [25] K 个一组翻转链表
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail = &mut head;
        let mut queue = vec![];
        let mut other;
        'a: loop {
            for _ in 0..k {
                if let Some(node) = tail {
                    tail = &mut node.next;
                } else {
                    break 'a;
                }
            }
            other = tail.take();
            queue.push(head);
            head = other;
            tail = &mut head;
        }

        let mut tail = head;
        while let Some(mut v) = queue.pop() {
            while let Some(mut node) = v {
                v = node.next.take();
                node.next = tail;
                tail = Some(node);
            }
        }

        tail
    }
}
// @lc code=end

